### ldm0's Blog Generator

+ This is a personal blog generator, not for others, written in Rust.
+ This program keeps me away from dirty blog frameworks. ^_^

+ HTML template follows specific grammer, and blogs in markdown also provided some metadata in specific garmmer. Yet they all pretty simple. I would like to give some introduction later. You can open files in `blogs` and `assets` to have a look.

+ Data Flow:

  ```
        +----------------+       +------------------+
        | html templates |       | blog in markdown |
        +----------------+       +------------------+
                |                           |
                +---------------------------+
                             |
                             v
                   +-------------------+
                   | my blog generator |
                   +-------------------+
                             |
         +--------------------------------------+
         v                   v                  v
    +----------+        +----------+       +---------+     +-------------------------------+
    | homepage |        | clusters |       |  blogs  |     | additional css and javascript |
    +----------+        +----------+       +---------+     +-------------------------------+
         |                   |                  |               |
         +--------------------------------------+               |
                             |                                  |
                             +<---------------------------------+
                             |
                             v
                      +-------------+
                      |   my blog   |
                      +-------------+
  ```

+ Test: `cargo test`
+ Build : `cargo build`