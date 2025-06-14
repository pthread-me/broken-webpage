# Finish article page templates [X]

1. Should be nearly identical to /home
2. I need a article select page, the css will take forever so not now, but it will be 
    "block" with a title and desc, I might need a summary column in the db but 
    pure text should be fine, i just dont know what the rendering process will look
    like yet, probably wont use a trigger, just iterate over all entries selecting title + summary then
    plug into, what for now will be this struct:
    ```rust
        #[derive(Template)]
        #[template=(path=".....")]
        struct Summary{
            title: &str,
            description: &str
        }
    ```
    slices should be fine cause we need to .to_owned right before dispatching to askama (at least
    until i figure out lifetimes) 


# Lazy SQL rendering + access [X]

1. set up sqlite db
    - Use Write ahead Logging
    ```sql
        PRAGMA journal_mode=WAL
    ```

    - Table Scheme?
    ```sql
        CREATE TABEL Articles(id INTEGER PRIMARY KEY ASC, title TEXT, html TEXT, md TEXT)
    ```

2. use pulldown-cmark to parse md -> html
3. Write md add to table, and delete existing html
4. I'm thinking a struct like:
    ```rust
        #[derive(FromRow)]
        struct Article{
            id: u64,
            title: &str,
            html: Option<String>,
            md: &str,
        };
        // I might change &str to String if the borrow checker keeps complaining
        // but I feel like it shouldn't cause prolly just clone title for template 
        // and md is for the derive to work
    ```
5. We read row (idk how yet, will check sqlx docs) if html == None then we use cmark to parse
    and update the db with the resultant html, otherwise, just use the prerendered html.
6. Finally when writing to the db, i need to delete existing html. Also this might be
    a bottle neck because of sqlite single reader thingy. But here's what im thinking.
    
    1. There will probably only be 1 connection to the db, i dont quite understand the specifics
        as i havent started implementing the db crate (or module) yet, but i doubt ill be pooling (or whatever)
    2. Any change that one connection makes will result in the exact same change as they both
        run the same parser on the same md, so if one fails just try again and in the next try its
        a read (cause the html would have been written). only time this changes is on
        md updates in which case, idk and idc atm.
        


