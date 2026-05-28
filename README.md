apt-query
=========

A tool for querying the apt database for available versions of packages, as JSON lines. It's primary use is for feeding
into other tools (e.g. [Paracrine](https://github.com/palfrey/paracrine/issues/92))

For example: `apt-query linux-image-*` to get all the kernel packages returns
```json
{"name":"linux-image-4.19.0-9-amd64","arch":"amd64","versions":["4.19.118-2+deb10u1"],"installed_version":null}
{"name":"linux-image-5.10.0-2-amd64","arch":"amd64","versions":["5.10.9-1"],"installed_version":null}
{"name":"linux-image-5.7.0-1-amd64","arch":"amd64","versions":["5.7.6-1"],"installed_version":null}
{"name":"linux-image-5.7.0-2-amd64","arch":"amd64","versions":["5.7.10-1"],"installed_version":null}
{"name":"linux-image-5.7.0-3-amd64","arch":"amd64","versions":["5.7.17-1"],"installed_version":null}
{"name":"linux-image-6.1.0-21-amd64","arch":"amd64","versions":["6.1.90-1"],"installed_version":"6.1.90-1"}
{"name":"linux-image-6.1.0-25-amd64","arch":"amd64","versions":["6.1.106-3"],"installed_version":"6.1.106-3"}
{"name":"linux-image-6.1.0-42-amd64","arch":"amd64","versions":["6.1.159-1"],"installed_version":null}
...
```
Note the patterns accepted are glob matches as per [fast-glob syntax](https://github.com/oxc-project/fast-glob#syntax)

Similar tools include:
- `apt cache` - which isn't exactly what you'd call "easily parseable" because of fun like the multi-line support.
- [`apt-show-versions`](https://salsa.debian.org/debian/apt-show-versions) - which doesn't tell you versions of not installed packages

We don't currently distribute binary images sadly, due to a lack of static build support for [libapt-pkg](https://salsa.debian.org/apt-team/apt),
but if anyone would like to have a go at that, I'd be very interested!