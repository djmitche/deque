## Comments

This does not compile.

The issue is juggling the links in insert_head, where we have three links to update.
In an earlier implementation, `List` was

```rust
struct List<T> {
    head: HalfLink<T>,
    tail: HalfLink<T>,
}
```

but this means that `self.head` must *always* be a valid HalfLink, leaving only
one other HalfLink to use for updating.  That's just not enough, so I adapted the struct to

```rust
struct List<T> {
    head: Option<HalfLink<T>>,
    tail: Option<HalfLink<T>>,
}
```

so allow mutation methods to `.take()` the links and use them, being careful to
set both `head` and `tail` back to `Some(_)` before returning.  But, this is
still not enough.
