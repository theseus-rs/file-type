use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_43: FileFormat = FileFormat {
    id: 43,
    source_type: SourceType::Linguist,
    name: "C++",
    extensions: &[
        "c++", "cc", "cp", "cpp", "cppm", "cxx", "h", "h++", "hh", "hpp", "hxx", "inc", "inl",
        "ino", "ipp", "ixx", "re", "tcc", "tpp", "txx",
    ],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
