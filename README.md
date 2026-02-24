# daily

## Local Firebase config

1. Copy `firebase-config.example.js` to `firebase-config.local.js`.
2. Fill in your Firebase values in `firebase-config.local.js`.
3. Open `index.html`.

`firebase-config.local.js` is git-ignored so your keys are not committed.

## GitHub Pages config

GitHub Pages loads Firebase config from `firebase-config.js`.

- Keep `firebase-config.js` in the repo for deployment.
- The app reads config from `window.__FIREBASE_CONFIG__` at startup.
- Task data is stored in Firebase Realtime Database, not browser local storage.

## Important security follow-up

Because the key was already committed, do this now:

1. Rotate/regenerate the exposed API key in Firebase/Google Cloud Console.
2. Restrict the new key to your site origins (HTTP referrer restrictions).
3. Restrict API usage to only the Firebase APIs you need.
4. Verify Firebase Realtime Database Security Rules are locked down.
5. Remove the old key from Git history if needed.

### Optional history cleanup command

If this repo is public and you want the key removed from commit history:

```bash
git filter-repo --path index.html --replace-text <(printf 'YOUR_EXPOSED_KEY==>REDACTED')
git push --force --all
```

Then revoke the old key anyway.
