from collections.abc import Iterable
from os import PathLike
from uuid import uuid4, UUID
import json

_box_container = {}

class BoxConversionError(Exception):
    """Base class for all box conversion errors."""
class BoxTypeError(BoxConversionError, TypeError):
    """Input is not a valid Box UUID."""
class BoxLookupError(BoxConversionError, LookupError):
    """Unknown Box UUID. Box may have been deleted."""

class Box:
    other_attr_mapping = {"_children": "_parents", "_parents": "_children", "_pointing": "_incoming", "_incoming": "_pointing"}

    def __init__(self, data, children: Iterable | None = None, id=None):
        """Creates a new Box containing arbitrary data. Optionally accepts children and a custom UUID."""
        self.id = id or uuid4()
        _box_container[self.id] = self

        self.data = data

        self._children: list[UUID] = []
        self._parents: list[UUID] = []

        self._incoming: list[UUID] = []  # Boxes that point to us
        self._pointing: list[UUID] = []  # Boxes we point to

        self.add(*children or [])

    # PUBLIC METHODS

    def add(self, *children):
        """Adds children boxes that link to us. Children may be a Box, UUID, UUID-like string, or raw data for a new Box. Inverse of `link`."""
        return self.connect(*children, outgoing=False, hard=True)

    def link(self, *parents):
        """Links to parents, making this box a child of them. Parents may be a Box, UUID, UUID-like string, or raw data for a new Box. Inverse of `add`."""
        return self.connect(*parents, outgoing=True, hard=True)

    def point(self, *pointing):
        """Creates soft outgoing references to other boxes (like foreign keys). Inverse of `receive`."""
        return self.connect(*pointing, outgoing=True, hard=False)

    def receive(self, *incoming):
        """Adds boxes that soft-reference us (like reverse foreign keys). Inverse of `point`."""
        return self.connect(*incoming, outgoing=False, hard=False)

    def get(self, item, deep=True):
        """Gets a child of this box matching the provided Box, UUID, or UUID-like string. Searches all descendants by default (deep=True)."""
        return self.get_boxes(item, deep=deep, outgoing=False, hard=True)

    def get_parents(self, item, deep=True):
        """Gets a parent of this box matching the provided Box, UUID, or UUID-like string. Searches all ancestors by default (deep=True)."""
        return self.get_boxes(item, deep=deep, outgoing=True, hard=True)

    def get_pointings(self, item, deep=True):
        """Gets a soft-referenced box matching the provided Box, UUID, or UUID-like string. Searches all outgoing soft references by default (deep=True)."""
        return self.get_boxes(item, deep=deep, outgoing=True, hard=False)

    def get_incomings(self, item, deep=True):
        """Gets a box that soft-references us matching the provided Box, UUID, or UUID-like string. Searches all incoming soft references by default (deep=True)."""
        return self.get_boxes(item, deep=deep, outgoing=False, hard=False)

    

    def find(self, item, deep=True, outgoing=False, hard=True):
        """Returns a list of boxes whose data matches `item`.
        deep=True: traverses all descendants, outgoing=False: traverses children/incoming connections, hard=True: traverses owned connections (parents/children).
        """
        return [box for box in self._traverse(deep=deep, outgoing=outgoing, hard=hard) if box.data == item]

    def set(self, data):
        """Sets this box's data to a new arbitrary value."""
        self.data = data
        return data

    def connect(self, *items, outgoing, hard):
        connecting_list = self._get_traversing_list(outgoing, hard)
        other_attr = Box.other_attr_mapping[connecting_list]

        items_added = []
        for item in items:
            try:
                other = Box.convert_to_box(item)
            except BoxConversionError:
                other = Box(item)

            if other.id not in connecting_list:
                connecting_list.append(other.id)
                getattr(other, other_attr).append(self.id)
                items_added.append(other)
        return items_added

    def search(self, where=lambda x: True, map_func=lambda x: x, deep=True, outgoing=False, hard=True):
        """Returns a generator of boxes matching the boolean lambda `where`, with each element transformed by `map_func`.
        deep=True: traverses all descendants, outgoing=False: traverses children/incoming connections, hard=True: traverses owned connections (parents/children).
        """
        return (map_func(box) for box in self._traverse(deep=deep, outgoing=outgoing, hard=hard) if where(box))

    def get_boxes(self, item, deep=True, outgoing=True, hard=True):
        """Gets box by UUID/UUID-like string/Box.
        deep=True: traverses all descendants, outgoing=False: traverses children/incoming connections, hard=True: traverses owned connections (parents/children).
        """

        item = Box.convert_to_box(item)
        if deep:
            return next((box for box in self._traverse(deep=True, outgoing=outgoing, hard=hard) if box.id == item.id), None)
        else:
            traversing_list = self._get_traversing_list(outgoing, hard)
            return item if item.id in traversing_list else None

    def delete(self, deep=True, outgoing=False, hard=True):
        """Deletes this box and optionally its descendants from the database, cleaning up all references.
        deep=True: traverses all descendants, outgoing=False: traverses children/incoming connections, hard=True: traverses owned connections (parents/children).
        """
        for box in list(self._traverse(deep=deep, outgoing=outgoing, hard=hard, yield_self=True)):
            for box_attr, reverse_attr in Box.other_attr_mapping.items():
                for box_id in getattr(box, box_attr):
                    getattr(_box_container[box_id], reverse_attr).remove(box.id)
            del _box_container[box.id]

    # PRIVATE METHODS

    def _get_traversing_list(self, outgoing, hard):
        if hard:
            return self._parents if outgoing else self._children
        else:
            return self._pointing if outgoing else self._incoming

    def _traverse(self, seen=None, yield_self=False, deep=True, outgoing=False, hard=True):
        if seen is None:
            seen = set()
        if self.id in seen:
            return
        seen.add(self.id)

        if yield_self:
            yield self

        traversing_list = self._get_traversing_list(outgoing, hard)
        for box_id in traversing_list:
            box = _box_container[box_id]
            if deep:
                yield from box._traverse(seen, True, True, outgoing, hard)
            else:
                yield box

    # DUNDER METHODS

    def __repr__(self):
        return f"Box({self.data}, _children={self._children}, _pointing={self._pointing})"

    def __contains__(self, item):
        try:
            item = Box.convert_to_box(item)
            return next((True for box in self._traverse(deep=False) if box.id == item.id), False)
        except BoxConversionError:
            return next((True for box in self._traverse(deep=False) if box.data == item), False)

    def __iter__(self):
        return self._traverse(deep=False)

    def __getitem__(self, item):
        item = self.get(item, deep=False)
        if item is None:
            raise KeyError("UUID/UUID-like string/matching Box exists in database but is not a direct child of this box.")
        return item

    # HELPER METHODS

    @staticmethod
    def convert_to_box(item: UUID | str | Box) -> Box:
        """Normalizes a UUID, UUID-like string, or Box to a Box instance via database lookup."""
        if isinstance(item, UUID):
            key = item
        elif isinstance(item, str):
            try:
                key = UUID(item)
            except ValueError:
                raise BoxTypeError("String given is not a valid UUID-like string. Check your formatting.")
        elif isinstance(item, Box):
            return item
        else:
            raise BoxTypeError("Data given is not a Box, UUID, or string. Check what you passed to the function.")

        try:
            return _box_container[key]
        except KeyError:
            raise BoxLookupError("Box does not exist with specified UUID. Maybe it has been deleted?")

    # GLOBAL METHODS

    @classmethod
    def create(cls, data, children: Iterable | None = None, id=None):
        """Creates a new Box containing data. Can optionally add children (i.e., other boxes that we own)."""
        return cls(data, children=children, id=id)

    @staticmethod
    def master_search(where=lambda x: True, map_func=lambda x: x):
        """Returns a list of all boxes in the database matching the boolean lambda `where`, with each element transformed by `map_func`."""
        return [map_func(box) for box in _box_container.values() if where(box)]

    @staticmethod
    def master_find(item):
        """Returns a list of all boxes in the database whose data matches `item`."""
        return [box for box in _box_container.values() if box.data == item]

    @staticmethod
    def masterget_boxes(item):
        """Returns the first box in the database whose data matches `item`, or None if not found."""
        return next((box for box in _box_container.values() if box.data == item), None)

    @staticmethod
    def delete_all():
        """Deletes all boxes from the database."""
        _box_container.clear()
    
    @staticmethod
    def to_dict():
        """Saves Deebee database to a dictionary reprersentation."""
        return {box.id: {"data": box.data, "children": box._children, "parents": box._parents,
                        "incoming": box._incoming, "pointing": box._pointing}
                        for box in _box_container}

    @staticmethod
    def from_dict(database):
        """Loads Deebee database dictionary representation into memory."""

        _box_container.clear()
        for box_id, box_info in database.items():
            box = Box.create(box_info["data"], id=box_id)
            box.add(*box_info["children"])
            box.link(*box_info["parents"])
            box.point(*box_info["pointing"])
            box.receive(*box_info["incoming"])

    @staticmethod
    def to_json(filename: int | str | bytes | PathLike[str] | PathLike[bytes]):
        """Saves Deebee database to an external JSON."""

        with open(filename, "w") as savefile:
            savefile.write(json.dumps(Box.to_dict()))

    @staticmethod
    def from_json(filename: int | str | bytes | PathLike[str] | PathLike[bytes]):
        """Loads external JSON Deebee database into memory."""
        global _box_container

        with open(filename, "r") as savefile:
            database = json.load(savefile)

        Box.from_dict(database)

if __name__ == "__main__":
    ...