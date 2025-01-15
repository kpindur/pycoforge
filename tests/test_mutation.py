import unittest
import random
from pycoforge import (
    PySubtreeMutation, PySizeFairMutation, PyPointMutation, PyConstantMutation,
    PyTreeGenotype, PyOperatorSampler
)


class TestMutationOperators(unittest.TestCase):
    def setUp(self):
        # Setup common objects needed for tests
        self.rng = random.Random(42)
        self.sampler = PyOperatorSampler(
            operators=["+", "x"],
            arity=[2, 0],
            weights=[0.5, 0.5]
        )
        self.tree = PyTreeGenotype.with_arena(["+", "x", "x"])
        self.tree.construct_children(self.sampler)

    def test_subtree_mutation(self):
        # Test constructor params
        mut = PySubtreeMutation(probability=0.7, depth_limits=(2, 5))
        self.assertEqual(mut.probability(), 0.7)
        self.assertEqual(mut.depth_limits(), (2, 5))

        # Test default constructor
        default_mut = PySubtreeMutation.default()
        self.assertTrue(0 <= default_mut.probability() <= 1)

        # Test mutation operation
        child = mut.variate(self.rng, self.tree, self.sampler)
        self.assertIsInstance(child, PyTreeGenotype)

        # Test invalid parameters
        with self.assertRaises(ValueError):
            PySubtreeMutation(probability=1.5, depth_limits=(2, 1))

    def test_size_fair_mutation(self):
        # Test constructor
        mut = PySizeFairMutation(probability=0.8, dynamic_limit=True)
        self.assertEqual(mut.probability(), 0.8)

        # Test default constructor
        default_mut = PySizeFairMutation.default()
        self.assertTrue(0 <= default_mut.probability() <= 1)

        # Test mutation operation
        child = mut.variate(self.rng, self.tree, self.sampler)
        self.assertIsInstance(child, PyTreeGenotype)

    def test_point_mutation(self):
        # Test constructor
        mut = PyPointMutation(probability=0.3)
        self.assertEqual(mut.probability(), 0.3)

        # Test default constructor
        default_mut = PyPointMutation.default()
        self.assertTrue(0 <= default_mut.probability() <= 1)

        # Test mutation operation
        child = mut.variate(self.rng, self.tree, self.sampler)
        self.assertIsInstance(child, PyTreeGenotype)

    def test_constant_mutation(self):
        # Test constructor with range limits
        mut = PyConstantMutation(
            probability=0.4,
            mutation_rate=0.1,
            range_limits=(-1.0, 1.0)
        )
        self.assertEqual(mut.probability(), 0.4)

        # Test default constructor
        default_mut = PyConstantMutation.default()
        self.assertTrue(0 <= default_mut.probability() <= 1)

        # Test mutation operation
        child = mut.variate(self.rng, self.tree, self.sampler)
        self.assertIsInstance(child, PyTreeGenotype)

        # Test invalid parameters
        with self.assertRaises(ValueError):
            PyConstantMutation(probability=1.5, mutation_rate=2.0)


if __name__ == '__main__':
    unittest.main()
