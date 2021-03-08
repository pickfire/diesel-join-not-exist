Diesel multi-join experiment
============================

Previous: https://github.com/pickfire/diesel-update-from
Next: https://github.com/pickfire/diesel-upsert

After getting footguns in sequelize and gorm, just want to try out diesel and
state of diesel.

I did not found a way to do left join where the other side does not exist, I
wanted to search a "has one" relationship where the parent table does not
have anything in the child table. But in gorm, I need to do a hack on `Joins`.

I think I started to get the hang of it. No wall crashing now, I just search
for "left join" and "null", then found `left_join` and `is_null` in the docs.
Tweak the original query, bingo! No surprise.

Seemed too easy now, maybe I need to try something harder next time. Probably
computed property or aggregation since I saw it could be done in diesel.

The example I tested out here have 3 struct (based on the previous ones).
sqlite was used for easy testing.

    +---------+     +---------+     +---------+
    | User    |<-+  | Post    |     | Comment |
    +---------+  |  +---------+     +---------+
    | id      |  |  | id      |     | id      |
    | name    |  |  | title   |     | body    |
    |         |  |  | body    |<----+ post_id |
    |         |  |  | user_id +--+--+ user_id |
    +---------+  |  +---------+  |  +---------+
                 +---------------+

I want to find all posts that does not have any comments. The query,

```rust
let query = posts::table
    .left_join(comments::table)
    .select(posts::title)
    .filter(comments::id.is_null());
```

Which results in the SQL query,

```sql
SELECT
  `posts`.`title`
 FROM (`posts`
 LEFT OUTER JOIN `comments`
   ON `comments`.`post_id` = `posts`.`id`)
WHERE `comments`.`id` IS NULL -- binds: []
```

## Get started

Rust, diesel_cli (with `sqlite` feature) is required.

```
$ diesel migration run
$ cargo run --bin init  # populate database
$ cargo run --bin hello  # multi join query
```
