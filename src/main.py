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

class Arrow:
    CHILD = ("_children", "_parents")
    PARENT = ("_parents", "_children")
    POINTING = ("_pointing", "_incoming")
    INCOMING = ("_incoming", "_pointing")

class Box:
    all_arrows = (Arrow.CHILD, Arrow.PARENT, Arrow.POINTING, Arrow.INCOMING)

    def __init__(self, data, children: Iterable | None = None, id=None):
        """Creates a new Box containing arbitrary data. Optionally accepts children and a custom UUID."""
        self.id = id or uuid4()
        _box_container[self.id] = self

        self.data = data

        self._children: list[UUID] = []
        self._parents: list[UUID] = []
        self._incoming: list[UUID] = [] # Boxes that point to us
        self._pointing: list[UUID] = [] # Boxes that we point to

        self.add(*children or [])

    # PUBLIC METHODS

    def add(self, *children):
        """Adds children boxes that link to us. Children may be a Box, UUID, UUID-like string, or raw data for a new Box. Inverse of `link`."""
        return self.connect(*children, arrow=Arrow.CHILD)

    def link(self, *parents):
        """Links to parents, making this box a child of them. Parents may be a Box, UUID, UUID-like string, or raw data for a new Box. Inverse of `add`."""
        return self.connect(*parents, arrow=Arrow.PARENT)

    def point(self, *pointing):
        """Creates soft outgoing references to other boxes (like foreign keys). Inverse of `receive`."""
        return self.connect(*pointing, arrow=Arrow.POINTING)

    def receive(self, *incoming):
        """Adds boxes that soft-reference us (like reverse foreign keys). Inverse of `point`."""
        return self.connect(*incoming, arrow=Arrow.INCOMING)

    def get(self, item, deep=True):
        """Gets a child of this box matching the provided Box, UUID, or UUID-like string. Searches all descendants by default (deep=True)."""
        return self.get_boxes(item, deep=deep, arrow=Arrow.CHILD)

    def get_parents(self, item, deep=True):
        """Gets a parent of this box matching the provided Box, UUID, or UUID-like string. Searches all ancestors by default (deep=True)."""
        return self.get_boxes(item, deep=deep, arrow=Arrow.PARENT)

    def get_pointings(self, item, deep=True):
        """Gets a soft-referenced box matching the provided Box, UUID, or UUID-like string. Searches all outgoing soft references by default (deep=True)."""
        return self.get_boxes(item, deep=deep, arrow=Arrow.POINTING)

    def get_incomings(self, item, deep=True):
        """Gets a box that soft-references us matching the provided Box, UUID, or UUID-like string. Searches all incoming soft references by default (deep=True)."""
        return self.get_boxes(item, deep=deep, arrow=Arrow.INCOMING)

    def find(self, item, deep=True, arrow=Arrow.CHILD):
        """Returns a list of boxes whose data matches `item`."""
        return [box for box in self._traverse(deep=deep, arrow=arrow) if box.data == item]

    def set(self, data):
        """Sets this box's data to a new arbitrary value."""
        self.data = data
        return data

    def connect(self, *items, arrow):
        connecting_list = getattr(self, arrow[0])
        other_attr = arrow[1]

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

    def search(self, where=lambda x: True, map_func=lambda x: x, deep=True, arrow=Arrow.CHILD):
        """Returns a generator of boxes matching the boolean lambda `where`, with each element transformed by `map_func`."""
        return (map_func(box) for box in self._traverse(deep=deep, arrow=arrow) if where(box))

    def get_boxes(self, item, deep=True, arrow=Arrow.CHILD):
        """Gets box by UUID/UUID-like string/Box."""
        item = Box.convert_to_box(item)

        if deep:
            return next((box for box in self._traverse(deep=True, arrow=arrow) if box.id == item.id), None)
        else:
            traversing_list = getattr(self, arrow[0])
            return item if item.id in traversing_list else None

    def delete(self, deep=True, arrow=Arrow.CHILD):
        """Deletes this box and optionally its descendants from the database, cleaning up all references."""
        for box in list(self._traverse(deep=deep, arrow=arrow, yield_self=True)):
            for box_attr, reverse_attr in Box.all_arrows:
                for box_id in getattr(box, box_attr):
                    getattr(_box_container[box_id], reverse_attr).remove(box.id)
            del _box_container[box.id]

    # PRIVATE METHODS
    def _traverse(self, seen=None, yield_self=False, deep=True, arrow=Arrow.CHILD):
        if seen is None:
            seen = set()

        if self.id in seen:
            return

        seen.add(self.id)

        if yield_self:
            yield self

        traversing_list = getattr(self, arrow[0])

        for box_id in traversing_list:
            box = _box_container[box_id]
            if deep:
                yield from box._traverse(seen, True, True, arrow)
            else:
                yield box

    # DUNDER METHODS

    def __repr__(self):
        return f"Box({self.data}, _children={self._children}, _pointing={self._pointing})"

    def __contains__(self, item):
        try:
            item = Box.convert_to_box(item)
            return next((True for box in self._traverse(deep=False, arrow=Arrow.CHILD) if box.id == item.id), False)
        except BoxConversionError:
            return next((True for box in self._traverse(deep=False, arrow=Arrow.CHILD) if box.data == item), False)

    def __iter__(self):
        return self._traverse(deep=False, arrow=Arrow.CHILD)

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
    def delete_all():
        """Deletes all boxes from the database."""
        _box_container.clear()

    @staticmethod
    def to_dict():
        """Saves Deebee database to a dictionary reprersentation."""
        return {
            str(box.id): {
                "data": box.data,
                "children": [str(id) for id in box._children],
                "parents": [str(id) for id in box._parents],
                "incoming": [str(id) for id in box._incoming],
                "pointing": [str(id) for id in box._pointing]
            }
            for box in _box_container.values()
        }

    @staticmethod
    def from_dict(database):
        _box_container.clear()
        
        for box_id, box_info in database.items():
            Box.create(box_info["data"], id=UUID(box_id))
        
        for box_id, box_info in database.items():
            box = _box_container[UUID(box_id)]
            box.add(*box_info["children"])
            box.point(*box_info["pointing"])
            # Connect will populate parents and incoming.

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
    Box.from_json("src/family.json")

    print(list(Box.master_search()))

    print(Box.master_find("Grandpa Joe")[0].find("Dad")[0].find("Bob"))