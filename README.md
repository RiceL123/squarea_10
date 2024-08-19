# Squarea_10

Clear tiles in a rectangle that add up to 10.

The scoring system is calculated by
```
score += n_tiles_popped + area_multiplier + combo_multiplier
```

### area_multiplier
The area_multiplier is defined as the number of tiles that area selected takes up.

### combo_multiplier
The combo mutliplier will activate and increment `n_combo` if the area selected intersects the previous area selected. The combo value is calculated by
```rust
let combo_bonus = match n_combo {
    0..6 => floor(n_combo.pow(1.5)),
    otherwise => n_combo * 2,
};
```
If the selected area does not intersect the previous area, `n_combo` will be set to zero.
