import unittest
import random

from pycoforge import PySubtreeCrossover, PyTreeGenotype, PyOperatorSampler


class TestSubtreeCrossover(unittest.TestCase):
    def setUp(self):
        self.rng = random.Random(42)

        self.tree1 = PyTreeGenotype.with_arena(["+", "x", "x"])
        self.sampler = PyOperatorSampler(
            operators=["+", "x"],
            arity=[2, 0],
            weights=[0.5, 0.5]
        )
        self.tree1.construct_children(self.sampler)

        self.tree2 = PyTreeGenotype.with_arena(["+", "+", "x", "x", "x"])
        self.tree2.construct_children(self.sampler)

    def test_constructor(self):
        crossover = PySubtreeCrossover(0.7)
        self.assertAlmostEqual(crossover.probability(), 0.7)

    def test_invalid_probability(self):
        with self.assertRaises(ValueError):
            PySubtreeCrossover(1.5)

    def test_default_constructor(self):
        crossover = PySubtreeCrossover.default()
        self.assertAlmostEqual(crossover.probability(), 0.7)

    def test_variate(self):
        crossover = PySubtreeCrossover(1.0)
        children = crossover.variate(
                self.rng,
                self.tree1, self.tree2,
                self.sampler
        )

        self.assertEqual(len(children), 2)

        for child in children:
            self.assertGreater(len(child.arena()), 0)
            children_map = child.children()

            self.assertEqual(len(children_map[0]), 2)


if __name__ == '__main__':
    unittest.main()
