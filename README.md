[![test badge]][test link] [![rust badge]][rust link] [![license badge]][license link]

# leaves 🍂

A place to leave your files.

**leaves 🍂** is a self-hostable file hosting service. Before uploading you need
a user account.

## Important routes

If open registration is enabled, then you can create an account by POSTing:

```http
POST /v1/users

{
  "email": "vivian@hellyer.dev",
}
```

You'll get back an API token and user ID:

```json
{
  "id": 4761,
  "token": "foo bar baz"
}
```

You can upload files by POSTing a body, with your API token and email:

```http request
POST /v1/files
Authorization: Basic vivian@hellyer.dev/token:foo bar baz

post file contents as the body
```

You'll get back a URL to use:

```json
"https://example.com/61xc90l"
```

Delete your file by DELETEing it:

```http request
DELETE /v1/files/61xc90l
Authorization: Basic vivian@hellyer.dev/token:foo bar baz
```

List your 100 most recent files:

```http request
GET /v1/users/@me/files?limit=100
Authorization: Basic vivian@hellyer.dev/token:foo bar baz
```

## Run it

`leaves` maintains a SQLite database and automatically runs migrations. All you
need to do is specify where you want your data to be kept, like maybe in a
volume:

```shell script
$ docker volume create leaves_data
$ docker run -itd --env-file leaves.env -v leaves_data:/data -p 10000:80 vivianis/leaves
```

[license badge]: https://img.shields.io/github/license/vivianhellyer/leaves?style=for-the-badge
[license link]: https://opensource.org/licenses/ISC
[rust badge]: https://img.shields.io/badge/Rust-1.41-93450a?style=for-the-badge
[rust link]: https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html
[test badge]: https://img.shields.io/github/workflow/status/vivianhellyer/leaves/Tests/master?style=for-the-badge
[test link]: https://github.com/vivianhellyer/leaves/actions
