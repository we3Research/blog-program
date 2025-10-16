use crate::*;

// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto",
            div { class: "text-center py-12",
                h1 { class: "text-4xl md:text-5xl font-bold mb-6 text-primary", "Welcome to Web3 Blog" }
                p { class: "text-xl mb-8", "A decentralized blogging platform built on Solana" }
                div { class: "card bg-base-200 shadow-xl p-6 mb-8",
                    h2 { class: "text-2xl font-semibold mb-4", "About this Platform" }
                    p { class: "mb-4", 
                        "This is a fully decentralized blogging platform built on the Solana blockchain. All blog posts are stored on-chain, making them immutable and censorship-resistant."
                    }
                    p { class: "mb-4",
                        "Each blog post is a Solana account containing metadata and content references. The platform demonstrates how web3 technologies can be used to create decentralized content applications."
                    }
                    div { class: "mt-6",
                        h3 { class: "text-lg font-semibold mb-2", "Features:" }
                        ul { class: "list-disc list-inside text-left max-w-md mx-auto",
                            li { "Immutable blog posts stored on Solana" }
                            li { "Decentralized content management" }
                            li { "Censorship-resistant publishing" }
                            li { "Transparent and verifiable content history" }
                        }
                    }
                }
                div { class: "card bg-base-200 shadow-xl p-6",
                    h2 { class: "text-2xl font-semibold mb-4", "Get Started" }
                    p { class: "mb-4", 
                        "Browse the blogs in the navigation panel to read existing posts. New posts can be created using the program's instruction interface."
                    }
                }
            }
        }
    }
}