from pycoforge import PyTreeGenotype, PyOperatorSampler


def test_tree_creation():
    arena = ["add", "5", "3"]
    children = {0: [1, 2]}

    tree = PyTreeGenotype(arena, children)

    assert tree.arena() == arena
    assert tree.children() == children


def test_tree_with_arena():
    arena = ["add", "5", "3"]
    tree = PyTreeGenotype.with_arena(arena)

    assert tree.arena() == arena
    assert tree.children() == {}


def test_subtree():
    arena = ["add", "mul", "2", "3", "5"]
    children = {0: [1, 4], 1: [2, 3]}

    tree = PyTreeGenotype(arena, children)

    assert tree.subtree(0) == 4
    assert tree.subtree(1) == 3


def test_str_representation():
    arena = ["add", "5", "3"]
    children = {0: [1, 2]}

    tree = PyTreeGenotype(arena, children)

    assert str(tree) != ""


def test_construct_children():
    arena = ["add", "5", "3"]
    children = {0: [1, 2]}

    tree = PyTreeGenotype.with_arena(arena)

    assert tree.arena() == arena

    sampler = PyOperatorSampler(
        operators=["add", "5", "3", "2"],
        arity=[2, 0, 0, 0],
        weights=[0.25, 0.25, 0.25, 0.25]
    )

    tree.construct_children(sampler)

    assert tree.children() == children
