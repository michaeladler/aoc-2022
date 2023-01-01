// src/num.c
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "helper.h"

typedef int32_t Data;

/* A node in a doubly-linked list. */
typedef struct Node {
    Data data;
    struct Node *next;
    struct Node *prev;
} Node;

/* Insert `node` after `dest`.
 *
 * Old: dest -> b
 * New: dest -> node -> b
 *
 * Complexity: O(1)
 */
static void insert_after(Node *dest, Node *node) {
    assert(dest != NULL);
    Node *b = dest->next;

    node->next = b;
    if (b)
        b->prev = node;

    node->prev = dest;
    dest->next = node;
}

/* Delete an existing node.
 *
 * NOTE: Consider freeing `node` after you're done with it.
 *
 * Before: `A -> node -> B`
 * After: `A -> B`
 *
 * Complexity: O(1)
 */
static void delete_node(const Node *node) {
    assert(node != NULL);
    Node *a = node->prev;
    Node *b = node->next;
    if (a)
        a->next = b;
    if (b)
        b->prev = a;
}

#define MAX_NODES 5000

int32_t mix(const int32_t items[], size_t n) {
    assert(n <= MAX_NODES);
    assert(n > 0);

    // remember original positions
    Node nodes[MAX_NODES];
    nodes[0].data = items[0];
    nodes[0].prev = &nodes[n - 1];
    nodes[0].next = &nodes[1];

    Node *zero_node = &nodes[0];

    for (size_t i = 1; i < n; i++) {
        Node *node = &nodes[i];
        node->data = items[i];
        if (node->data == 0) {
            zero_node = node;
        }
        node->prev = &nodes[i - 1];
        node->next = &nodes[(i + 1) % n];
    }

    // n-1 because we remove the element before rotating the list to avoid funny corner-cases
    int32_t mod = n - 1;
    for (size_t i = 0; i < n; i++) {
        Node *current = &nodes[i];

        int32_t steps = current->data % mod;
        if (steps == 0) {
            continue;
        }

        delete_node(current);
        Node *dest = current->prev;

        if (steps >= 0) {
            for (int32_t j = 0; j < steps; j++) {
                dest = dest->next;
            }
        } else {
            for (int32_t j = 0; j < -steps; j++) {
                dest = dest->prev;
            }
        }
        insert_after(dest, current);
    }

    int32_t result = 0;
    {  // the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers after the value `0`,
        // wrapping around the list as necessary
        Node *current = zero_node;
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 1000; j++) {
                current = current->next;
            }
            result += current->data;
        }
    }

    return result;
}
