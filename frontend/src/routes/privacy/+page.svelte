<script lang="ts">
    import Footer from '$lib/components/Footer.svelte';
    import { ArrowLeft } from 'lucide-svelte';

    const lastUpdated = 'March 13, 2026';
    const appName = 'WRF';
</script>

<svelte:head>
    <title>Privacy Policy</title>
</svelte:head>

<div class="flex w-full flex-1 flex-col items-center justify-center p-8 md:p-12">
    <div class="w-full max-w-4xl space-y-12">
        <header class="border-surface-500/30 mb-12 border-b pb-8">
            <div class="mb-4 flex items-center">
                <a href="/app" class="btn preset-tonal-surface flex items-center gap-2">
                    <ArrowLeft />
                    <span>Back to Home</span>
                </a>
            </div>
            <h1 class="h1 mb-2">Privacy Policy</h1>
            <p class="text-sm italic opacity-60">Effective Date: {lastUpdated}</p>
        </header>

        <main class="space-y-12">
            <section class="space-y-4">
                <h2 class="h2">1. Introduction</h2>
                <p class="opacity-90">
                    This Privacy Policy describes how <strong>{appName}</strong>
                    ("we," "our," or "us") collects, uses, processes, and protects your information when
                    you access and use our application. We are committed to data minimization and do not
                    store Personally Identifiable Information (PII) such as email addresses or real names
                    within our primary databases.
                </p>
            </section>

            <section class="space-y-4">
                <h2 class="h2">2. Information Collection and Processing</h2>
                <p class="opacity-90">
                    Our application employs a decentralized identity architecture. Information
                    collection is bifurcated between our third-party authentication provider and our
                    application database:
                </p>

                <div class="card preset-outlined-surface-200-800 space-y-6 p-6">
                    <div>
                        <h3 class="h3 mb-2 flex items-center gap-2">
                            <span class="badge preset-filled-secondary-500">Auth</span>
                            <span class="h4">Authentication Data</span>
                        </h3>
                        <p class="text-sm opacity-80">
                            Account creation and authentication are handled securely by our OpenID
                            Connect (OIDC) provider, <strong>Clerk</strong>
                            . While Clerk processes your login credentials (e.g., email address), this
                            information is not transmitted to, nor stored within, our primary application
                            database. Clerk issues a cryptographic token representing your session.
                        </p>
                    </div>
                    <hr class="opacity-20" />
                    <div>
                        <h3 class="h3 mb-2 flex items-center gap-2">
                            <span class="badge preset-filled-primary-500">Database</span>
                            <span class="h4">Application Data</span>
                        </h3>
                        <p class="text-sm opacity-80">
                            Our primary database (SpacetimeDB) associates your session with an
                            anonymized, cryptographically generated <strong>Identity</strong>
                            . This identifier contains no PII and serves as the sole mechanism for recognizing
                            your account across sessions.
                        </p>
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="h2">3. Data We Store</h2>
                <p class="opacity-90">
                    We strictly limit data retention to information required for core application
                    functionality. The following data is stored within our databases:
                </p>

                <ul class="ml-4 list-inside list-disc space-y-2 opacity-90">
                    <li>
                        <strong>In-Game Username:</strong>
                        The public alias you elect to display within the application.
                    </li>
                    <li>
                        <strong>Warframe ID:</strong>
                        An identifier utilized exclusively for verifying your in-game account status.
                    </li>
                </ul>
            </section>

            <section class="space-y-4">
                <h2 class="h2">4. Third-Party Service Providers</h2>
                <p class="opacity-90">
                    We engage the following third-party data processor to facilitate secure
                    authentication. This provider is bound by their respective privacy policy and
                    security standards:
                </p>

                <div class="card preset-tonal-surface p-6">
                    <ul class="space-y-4">
                        <li class="flex items-center justify-between">
                            <div>
                                <p class="font-bold">Clerk</p>
                                <p class="text-sm opacity-60">
                                    Identity & Authentication Processing
                                </p>
                            </div>
                            <a
                                class="anchor text-sm"
                                href="https://clerk.com/privacy"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                View Policy
                            </a>
                        </li>
                    </ul>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="h2">5. User Rights and Data Deletion</h2>
                <p class="opacity-90">
                    You retain complete control over your application data. You may initiate an
                    account deletion request at any time. Because we do not store PII, account
                    deletion consists of severing the link between your OIDC identity and your
                    in-game username, followed by the immediate purging of your associated
                    application data (Username and Warframe ID) from our systems.
                </p>
            </section>

            <section class="space-y-4">
                <h2 class="h2">6. Modifications to this Policy</h2>
                <p class="opacity-90">
                    We reserve the right to modify this Privacy Policy at any time. Any changes will
                    be reflected by an updated "Effective Date" at the top of this document.
                    Continued use of the application following such modifications constitutes your
                    acknowledgment of the updated policy.
                </p>
            </section>

            <section class="space-y-4">
                <h2 class="h2">7. Contact Information</h2>
                <p class="opacity-90">
                    If you have any questions regarding this Privacy Policy or our data practices,
                    please contact us via email at:
                    <a href="mailto:support@mettwasser.xyz" class="anchor">
                        support@mettwasser.xyz
                    </a>
                </p>
            </section>
        </main>

        <Footer />
    </div>
</div>
