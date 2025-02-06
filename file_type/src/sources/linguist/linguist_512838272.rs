use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_512838272: FileFormat = FileFormat {
    id: 512_838_272,
    source_type: SourceType::Linguist,
    name: "MDX",
    extensions: &["mdx"],
    media_types: &["text/x-gfm"],
    signatures: &[],
    related_formats: &[],
};
