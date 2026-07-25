# Image drop-in manifest

Replace any file below with your own photo, keeping the exact filename AND
extension. `object-cover` crops any resolution to the slot — no code change,
no resolution in the filename.

| File | Section / component | Rendered as | Recommended aspect | Min resolution |
| --- | --- | --- | --- | --- |
| `hero.jpg` | Hero (`src/app/sections/hero.rs`) | Full-bleed background, `object-cover` | Landscape ~3:2 | ≥1920×1280 |
| `about-portrait.jpg` | About (`src/app/sections/about.rs`) | Arched portrait, `aspect-[500/650] object-cover` | Portrait ~4:5 | ≥1000×1300 |
| `gallery-01.jpg` | Gallery (`src/app/sections/gallery.rs`), row 1 left tile | Grid tile, `object-cover` | Landscape ~2:1 | ≥1600px on the long edge |
| `gallery-02.jpg` | Gallery, row 1 right tile | Grid tile, `object-cover` | Square ~1:1 | ≥1200px on the long edge |
| `gallery-03.jpg` | Gallery, row 2 left tile | Grid tile, `object-cover` | Portrait ~3:4 | ≥1200px on the long edge |
| `gallery-04.jpg` | Gallery, row 2 right tile | Grid tile, `object-cover` | Landscape ~3:2 | ≥1600px on the long edge |
| `gallery-05.jpg` | Gallery, row 3 left tile | Grid tile, `object-cover` | Landscape ~4:3 | ≥1200px on the long edge |
| `gallery-06.jpg` | Gallery, row 3 right tile | Grid tile, `object-cover` | Landscape ~4:3 | ≥1200px on the long edge |
| `avatar-elena.jpg` | Testimonial (`src/app/sections/testimonial.rs`) | Round avatar, `rounded-full object-cover` | Square 1:1 | ≥240×240 |

Each aspect ratio above is the shape of that tile's actual container at
desktop width (the widest, most demanding crop each image has to survive);
`object-cover` fills the container from any source resolution at or above
the minimum, so a slightly different source aspect is fine — it will just
crop from the center.

## Not a drop-in slot

The Instagram grid (`src/app/sections/instagram.rs`, `#connect` section) is
**not** listed above. Its tiles render the live Instagram feed via
`get_feed()` (Phase 4 — server-cached real IG media) and are replaced by
that pipeline, not by files in this directory. Until Phase 4 ships, that
grid renders a static tinted placeholder with no image files at all.
