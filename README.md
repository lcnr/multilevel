# Multilevel entry APIs using generics

This blog post was inspired by a conversation with Ana Hoverbear [on twitter](https://twitter.com/lcnr7/status/1311694821015400449).

One of the more interesting patterns in rust is the `entry` API, which is a way to cleanly deal with potentially missing values in `HashMap`s.

Here is an example taken from [its documentation](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html#method.entry):

```rust
let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    *counter += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

While this is already quite nice, there are some cases where it does not feel sufficient.

Let's pretend you are interested in animals and want to count how often a given species ate a fruit in a given location.

We might want to use a `HashMap<Animal, Vec<HashMap<Fruit, usize>>>` for this, where we enumerate locations using positive integers.

So if we want to know how often a pig ate an apple in the sixth location,
we could query this using
```rust
let count = data
    .get(&Animal("pig"))
    .and_then(|pigs| pigs.get(6))
    .and_then(|pig_in_sixth| pig_in_sixth.get(&Fruit("apple")))
    .copied()
    .unwrap_or(0);
```
And to insert new data, we can use
```rust
let pigs = data.entry(Animal("pig")).or_default();
let pigs_in_sixth = if let Some(pigs_in_sixth) = pigs.get_mut(6) {
    pigs_in_sixth
} else {
    pigs.resize_with(6 + 1, Default::default);
    pigs.last_mut().unwrap()
};
*pigs_in_sixth.entry(Fruit("apple")).or_default() += 1;
```
This is really cumbersome however, so a nicer way might be something like
```rust
let data: HashMap<Animal, Vec<HashMap<Fruit, usize>>>> = HashMap::new();

*data
    .deep_entry((Animal("pig"), (6, Fruit("apply"))))
    .or_default() += 1;

assert_eq!(
    *data
        .deep_entry((Animal("pig"), (6, Fruit("apply"))))
        .or_insert(0),
    1
);
```
## Getting started

When thinking about implementing this we are only going to implement the bare minimum to show that this works.
