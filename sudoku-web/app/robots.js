export default function robots() {
  return {
    rules: {
      userAgent: "*",
      allow: ["/"],
      disallow: ["/api/", "/_next/"],
      crawlDelay: 1,
    },
    sitemap: "https://xodoku.com/sitemap.xml",
  };
}
