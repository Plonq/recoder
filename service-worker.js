const cacheName = 'recoder-v1';
const filesToCache = [
  '/',
  '/index.html',
  '/recoder.js',
  '/recoder_bg.wasm',
  '/pkg/recoder.js',
  '/pkg/recoder_bg.wasm',
];


self.addEventListener("install", (e) => {
  console.debug("[Service Worker] Install");
  e.waitUntil(
    (async () => {
      const cache = await caches.open(cacheName);
      console.debug("[Service Worker] Caching all: app shell and content");
      await cache.addAll(filesToCache);
    })()
  );
});

self.addEventListener("activate", (e) => {
  console.debug("[Service Worker] Clearing old cache");
  e.waitUntil(
    caches.keys().then((keyList) => {
      return Promise.all(
        keyList.map((key) => {
          if (key === cacheName) {
            return;
          }
          return caches.delete(key);
        })
      );
    })
  );
});

self.addEventListener("fetch", (e) => {
  e.respondWith(
    (async () => {
      const r = await caches.match(e.request);
      console.debug(`[Service Worker] Fetching resource: ${e.request.url}`);
      if (r) {
        return r;
      }
      const response = await fetch(e.request);
      const cache = await caches.open(cacheName);
      console.debug(`[Service Worker] Caching new resource: ${e.request.url}`);
      cache.put(e.request, response.clone());
      return response;
    })()
  );
});
