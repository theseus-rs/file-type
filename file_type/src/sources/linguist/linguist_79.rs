use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_79: FileFormat = FileFormat {
    id: 79,
    source_type: SourceType::Linguist,
    name: "Cython",
    extensions: &["pxd", "pxi", "pyx"],
    media_types: &["text/x-cython"],
    signatures: &[],
    related_formats: &[],
};
