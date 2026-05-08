import pytest
import json
import tempfile
import os
from uuid import uuid4, UUID
from main import Box, BoxTypeError, BoxLookupError, BoxConversionError, Arrow

@pytest.fixture(autouse=True)
def clean():
    Box.delete_all()
    yield
    Box.delete_all()

# ── CREATION ──

def test_create_basic():
    b = Box.create("Dog")
    assert b.data == "Dog"

def test_create_with_children():
    b = Box.create("Dog", ["CarKey", "Chihuahua"])
    children = list(b)
    assert len(children) == 2
    assert any(c.data == "CarKey" for c in children)

def test_create_stores_in_container():
    b = Box.create("Dog")
    assert Box.convert_to_box(b.id) is b

def test_id_is_uuid():
    b = Box.create("test")
    assert isinstance(b.id, UUID)

def test_create_custom_id():
    custom_id = uuid4()
    b = Box.create("test", id=custom_id)
    assert b.id == custom_id

# ── ADD ──

def test_add_raw_data():
    b = Box.create("Parent")
    b.add("Child")
    assert any(c.data == "Child" for c in b)

def test_add_box():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert child in parent

def test_add_uuid():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child.id)
    assert child in parent

def test_add_uuid_string():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(str(child.id))
    assert child in parent

def test_add_no_duplicates():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    parent.add(child)
    assert len(parent._children) == 1

def test_add_sets_parent():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert parent.id in child._parents

def test_add_returns_added():
    parent = Box.create("Parent")
    child = Box.create("Child")
    result = parent.add(child)
    assert result == [child]

def test_add_returns_empty_on_duplicate():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert parent.add(child) == []

# ── LINK ──

def test_link_basic():
    parent = Box.create("Parent")
    child = Box.create("Child")
    child.link(parent)
    assert child in parent

def test_link_no_duplicates():
    parent = Box.create("Parent")
    child = Box.create("Child")
    child.link(parent)
    child.link(parent)
    assert len(child._parents) == 1

def test_link_sets_child():
    parent = Box.create("Parent")
    child = Box.create("Child")
    child.link(parent)
    assert child.id in parent._children

def test_link_raw_data_creates_box():
    child = Box.create("Child")
    child.link("NewParent")
    assert any(Box.convert_to_box(pid).data == "NewParent" for pid in child._parents)

# ── POINT / RECEIVE ──

def test_point_creates_soft_outgoing():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    assert b.id in a._pointing
    assert a.id in b._incoming

def test_point_does_not_create_hard_link():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    assert b.id not in a._children
    assert a.id not in b._parents

def test_point_no_duplicates():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    a.point(b)
    assert len(a._pointing) == 1

def test_receive_basic():
    a = Box.create("A")
    b = Box.create("B")
    a.receive(b)
    assert b.id in a._incoming
    assert a.id in b._pointing

def test_receive_no_duplicates():
    a = Box.create("A")
    b = Box.create("B")
    a.receive(b)
    a.receive(b)
    assert len(a._incoming) == 1

# ── CONNECT ──

def test_connect_arrow_child():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.connect(child, arrow=Arrow.CHILD)
    assert child in parent

def test_connect_arrow_pointing():
    a = Box.create("A")
    b = Box.create("B")
    a.connect(b, arrow=Arrow.POINTING)
    assert b.id in a._pointing

def test_connect_returns_added():
    parent = Box.create("Parent")
    child = Box.create("Child")
    result = parent.connect(child, arrow=Arrow.CHILD)
    assert result == [child]

# ── GET ──

def test_get_direct_child():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert parent.get(child, deep=False) is child

def test_get_not_direct_child_returns_none():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    assert parent.get(grandchild, deep=False) is None

def test_get_deep():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    assert parent.get(grandchild, deep=True) is grandchild

def test_get_missing_raises():
    parent = Box.create("Parent")
    with pytest.raises(BoxLookupError):
        parent.get(uuid4())

# ── GET PARENTS ──

def test_get_parents_direct():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert child.get_parents(parent, deep=False) is parent

def test_get_parents_deep():
    grandparent = Box.create("Grandparent")
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandparent.add(parent)
    parent.add(child)
    assert child.get_parents(grandparent, deep=True) is grandparent

# ── GET POINTINGS ──

def test_get_pointings_direct():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    assert a.get_pointings(b, deep=False) is b

def test_get_pointings_deep():
    a = Box.create("A")
    b = Box.create("B")
    c = Box.create("C")
    a.point(b)
    b.point(c)
    assert a.get_pointings(c, deep=True) is c

# ── GET INCOMINGS ──

def test_get_incomings_direct():
    a = Box.create("A")
    b = Box.create("B")
    b.point(a)
    assert a.get_incomings(b, deep=False) is b

def test_get_incomings_deep():
    a = Box.create("A")
    b = Box.create("B")
    c = Box.create("C")
    c.point(b)
    b.point(a)
    assert a.get_incomings(c, deep=True) is c

# ── GET BOXES ──

def test_get_boxes_custom_arrow():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert parent.get_boxes(child, deep=False, arrow=Arrow.CHILD) is child

# ── GETITEM ──

def test_getitem_direct_child():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert parent[child] is child

def test_getitem_not_child_raises_keyerror():
    parent = Box.create("Parent")
    stranger = Box.create("Stranger")
    with pytest.raises(KeyError):
        parent[stranger]

def test_getitem_grandchild_raises_keyerror():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    with pytest.raises(KeyError):
        parent[grandchild]

# ── CONTAINS ──

def test_contains_direct_child():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    assert child in parent

def test_contains_not_child():
    parent = Box.create("Parent")
    stranger = Box.create("Stranger")
    assert stranger not in parent

def test_contains_grandchild_is_false():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    assert grandchild not in parent

def test_contains_by_data():
    parent = Box.create("Parent")
    parent.add("CarKey")
    assert "CarKey" in parent

def test_contains_nonexistent_data():
    parent = Box.create("Parent")
    assert "ghost" not in parent

def test_contains_soft_not_in_hard():
    parent = Box.create("Parent")
    child = Box.create("Child")
    child.point(parent)
    assert child not in parent

# ── ITER ──

def test_iter_direct_children_only():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    result = list(parent)
    assert child in result
    assert grandchild not in result

def test_iter_excludes_soft():
    parent = Box.create("Parent")
    soft = Box.create("Soft")
    soft.point(parent)
    assert soft not in list(parent)

# ── SEARCH ──

def test_search_default():
    parent = Box.create("Parent")
    parent.add("A", "B", "C")
    assert len(list(parent.search())) == 3

def test_search_where():
    parent = Box.create("Parent")
    parent.add("Apple", "Banana", "Apricot")
    result = list(parent.search(where=lambda b: b.data.startswith("A")))
    assert len(result) == 2

def test_search_map():
    parent = Box.create("Parent")
    parent.add("hello")
    result = list(parent.search(map_func=lambda b: b.data.upper()))
    assert "HELLO" in result

def test_search_deep():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    assert grandchild in list(parent.search(deep=True))

def test_search_shallow():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    assert grandchild not in list(parent.search(deep=False))

def test_search_arrow_parent():
    grandparent = Box.create("Grandparent")
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandparent.add(parent)
    parent.add(child)
    result = list(child.search(deep=True, arrow=Arrow.PARENT))
    assert parent in result
    assert grandparent in result

def test_search_arrow_pointing():
    a = Box.create("A")
    b = Box.create("B")
    c = Box.create("C")
    a.point(b)
    b.point(c)
    result = list(a.search(deep=True, arrow=Arrow.POINTING))
    assert b in result
    assert c in result

def test_search_arrow_incoming():
    a = Box.create("A")
    b = Box.create("B")
    c = Box.create("C")
    b.point(a)
    c.point(b)
    result = list(a.search(deep=True, arrow=Arrow.INCOMING))
    assert b in result
    assert c in result

# ── FIND ──

def test_find_basic():
    parent = Box.create("Parent")
    parent.add("Apple", "Banana", "Apple")
    result = parent.find("Apple")
    assert len(result) == 2

def test_find_deep():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Target")
    child.add(grandchild)
    parent.add(child)
    result = parent.find("Target", deep=True)
    assert grandchild in result

def test_find_shallow():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Target")
    child.add(grandchild)
    parent.add(child)
    result = parent.find("Target", deep=False)
    assert grandchild not in result

# ── DELETE ──

def test_delete_single():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    child.delete(deep=False)
    assert child.id not in Box.master_search(where=lambda x: x.id == child.id)
    assert child.id not in parent._children

def test_delete_deep():
    parent = Box.create("Parent")
    child = Box.create("Child")
    grandchild = Box.create("Grandchild")
    child.add(grandchild)
    parent.add(child)
    parent.delete(deep=True)
    assert Box.master_find("Child") == []
    assert Box.master_find("Grandchild") == []

def test_delete_cleans_references():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    child.delete(deep=False)
    assert child.id not in parent._children

def test_delete_soft_edges():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    a.delete(deep=False)
    assert a.id not in b._incoming

# ── CYCLES ──

def test_hard_cycle_does_not_recurse():
    a = Box.create("A")
    b = Box.create("B")
    a.add(b)
    b.add(a)
    assert len(list(a.search(deep=True))) == 1

def test_soft_cycle_does_not_recurse():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    b.point(a)
    assert len(list(a.search(deep=True, arrow=Arrow.POINTING))) == 1

# ── CONVERT TO BOX ──

def test_convert_box_passthrough():
    b = Box.create("test")
    assert Box.convert_to_box(b) is b

def test_convert_uuid():
    b = Box.create("test")
    assert Box.convert_to_box(b.id) is b

def test_convert_uuid_string():
    b = Box.create("test")
    assert Box.convert_to_box(str(b.id)) is b

def test_convert_invalid_string_raises():
    with pytest.raises(BoxTypeError):
        Box.convert_to_box("not-a-uuid")

def test_convert_invalid_type_raises():
    with pytest.raises(BoxTypeError):
        Box.convert_to_box(12345)

def test_convert_missing_uuid_raises():
    with pytest.raises(BoxLookupError):
        Box.convert_to_box(uuid4())

# ── MASTER SEARCH / MASTER FIND ──

def test_master_search_by_lambda():
    b = Box.create("unique_xyz")
    assert b in Box.master_search(where=lambda x: x.data == "unique_xyz")

def test_master_search_missing_returns_empty():
    assert Box.master_search(where=lambda x: x.data == "nonexistent") == []

def test_master_search_map():
    Box.create("hello")
    result = Box.master_search(where=lambda x: x.data == "hello", map_func=lambda x: x.data.upper())
    assert result == ["HELLO"]

def test_master_find_basic():
    b = Box.create("unique_xyz")
    assert b in Box.master_find("unique_xyz")

def test_master_find_missing_returns_empty():
    assert Box.master_find("nonexistent") == []

def test_master_find_multiple():
    a = Box.create("same")
    b = Box.create("same")
    result = Box.master_find("same")
    assert a in result and b in result

# ── SET ──

def test_set_data():
    b = Box.create("old")
    b.set("new")
    assert b.data == "new"

def test_set_returns_data():
    b = Box.create("old")
    assert b.set("new") == "new"

# ── DELETE ALL ──

def test_delete_all():
    Box.create("A")
    Box.create("B")
    Box.delete_all()
    assert Box.master_find("A") == []

# ── TO DICT / FROM DICT ──

def test_to_dict_structure():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    d = Box.to_dict()
    assert str(parent.id) in d
    assert str(child.id) in d
    assert str(child.id) in d[str(parent.id)]["children"]

def test_to_dict_uuids_are_strings():
    Box.create("test")
    d = Box.to_dict()
    for key, val in d.items():
        assert isinstance(key, str)
        for id_list in [val["children"], val["parents"], val["pointing"], val["incoming"]]:
            assert all(isinstance(i, str) for i in id_list)

def test_from_dict_restores_boxes():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    d = Box.to_dict()
    Box.delete_all()
    Box.from_dict(d)
    assert Box.master_find("Parent") != []
    assert Box.master_find("Child") != []

def test_from_dict_restores_relationships():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    parent_id = parent.id
    child_id = child.id
    d = Box.to_dict()
    Box.delete_all()
    Box.from_dict(d)
    restored_parent = Box.convert_to_box(parent_id)
    assert child_id in restored_parent._children

def test_from_dict_restores_soft_edges():
    a = Box.create("A")
    b = Box.create("B")
    a.point(b)
    a_id = a.id
    b_id = b.id
    d = Box.to_dict()
    Box.delete_all()
    Box.from_dict(d)
    restored_a = Box.convert_to_box(a_id)
    assert b_id in restored_a._pointing

# ── TO JSON / FROM JSON ──

def test_to_json_and_from_json():
    parent = Box.create("Parent")
    child = Box.create("Child")
    parent.add(child)
    parent_id = parent.id
    child_id = child.id

    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        fname = f.name
    try:
        Box.to_json(fname)
        Box.delete_all()
        Box.from_json(fname)
        restored = Box.convert_to_box(parent_id)
        assert child_id in restored._children
    finally:
        os.unlink(fname)

def test_to_json_is_valid_json():
    Box.create("test")
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        fname = f.name
    try:
        Box.to_json(fname)
        with open(fname) as f:
            data = json.load(f)
        assert isinstance(data, dict)
    finally:
        os.unlink(fname)