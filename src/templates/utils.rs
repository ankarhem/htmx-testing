pub struct Link {
    pub href: &'static str,
    pub text: &'static str,
}

pub const LINKS: [Link; 1] = [Link {
    href: "/blog",
    text: "Blog",
}];
