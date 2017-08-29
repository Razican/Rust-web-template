//! Error module.

#![allow(unused_doc_comment, box_pointers)]

error_chain!{
    foreign_links {
        Io(::std::io::Error);
    }
}
