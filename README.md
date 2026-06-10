# Deebee
A super tiny graph database built on a philosophy of simplicity, flexibility, and sugar. It is written in Rust for speed, stability, and idiomatic design, although the API is streamlined greatly for use in other languages, such as Python. Deebee does not have a query language by design, rather the progamming language itself filling that gap with each method made to read like standard English. It is written in X lines of easy-to-understand code (partially for design and partially because I'm an newbie at Rust), entirely contained in one `lib.rs` file.

Deebee consists of only four primitives: the Graph, storing nodes, the Node, storing values and connecting to other nodes, the Arrow, the connection to the other nodes, and the Value, dynamic data stored by the Node.

THIS DATABASE'S CODEBASE WAS RUSHED TO COMPLETION IN A FEW HOURS AFTER DEBATING THE FIRST 75% FOR WEEKS SO IF THERE ARE ANY WEIRD DESIGN DECISIONS THEN MAYBE I WILL FIX THEM EVENTUALLY