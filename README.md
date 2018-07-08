# pxscrape: Replacing 500px legacy API features

`pxscrape` is a small utility to provide 500px photostream information, since the 500px API is now deprecated.

## Usage:
`pxscrape [username] [output file]`

## Output:
```json
[
  {
    "small_url": "https://...",
    "medium_url": "https://...",
    "large_url": "https://...",
    "title": "My photo title",
    "location": "Photo location (actually the photo description)",
    "date": "Date of photo capture"
  },
  ...
]
```