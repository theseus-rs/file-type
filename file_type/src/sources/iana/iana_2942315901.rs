use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2942315901: FileFormat = FileFormat {
    id: 2_942_315_901,
    source_type: SourceType::Iana,
    name: "vnd.dpgraph",
    extensions: &[],
    media_types: &["application/vnd.dpgraph"],
    internal_signatures: &[],
    related_formats: &[],
};
