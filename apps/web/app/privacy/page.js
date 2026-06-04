export const metadata = {
  title: "Privacy Policy",
  description:
    "Read how Xodoku handles Sudoku images, puzzle data, analytics and privacy.",
  alternates: {
    canonical: "/privacy",
  },
  openGraph: {
    title: "Privacy Policy | Xodoku",
    description:
      "Read how Xodoku handles Sudoku images, puzzle data, analytics and privacy.",
    url: "/privacy",
  },
};

export default function Privacy() {
  return (
    <main className="mx-auto max-w-3xl py-6 sm:py-8">
        <h1 className="text-3xl font-bold mb-6">Privacy Policy</h1>

        <div className="article-content">
          <p className="text-gray-600 mb-6">
            Your privacy is important to us. Learn how we collect, use, and
            protect your data.
          </p>

          <p className="text-sm text-gray-500 mb-8">
            Last Updated: Aug 15, 2025
          </p>

          <div className="space-y-8">
            <section>
              <p className="mb-4">
                At Xodoku, we take your privacy seriously. This Privacy Policy
                explains how we collect, use, disclose, and safeguard your
                information when you use our service to scan, play, and generate
                puzzles.
              </p>

              <p className="mb-6">
                By using Xodoku, you agree to the collection and use of
                information in accordance with this policy.
              </p>
            </section>

            <section>
              <h2 className="text-2xl font-semibold mb-4">
                Information We Collect
              </h2>
              <p className="mb-4">
                Sudoku image scanning, playing, and generation run locally in
                your browser. Sudoku images are not uploaded to our server.
              </p>
            </section>

            <section>
              <h2 className="text-2xl font-semibold mb-4">
                Third-Party Services
              </h2>
              <p className="mb-4">
                We may employ third-party companies and individuals to
                facilitate our service, provide service-related services, or
                assist us in analyzing how our service is used. These third
                parties have access to your information only to perform these
                tasks on our behalf and are obligated not to disclose or use it
                for any other purpose.
              </p>
            </section>

            <section>
              <h2 className="text-2xl font-semibold mb-4">Contact Us</h2>
              <p className="mb-4">
                If you have any questions about this Privacy Policy, please
                contact us:
              </p>
              <ul className="list-disc pl-6 space-y-2 mb-4">
                <li>By email: rocky.zheng314@gmail.com</li>
                <li>By visiting the contact section on our website</li>
              </ul>
            </section>
          </div>
        </div>
    </main>
  );
}
