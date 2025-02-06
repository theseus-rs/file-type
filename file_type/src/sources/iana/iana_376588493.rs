use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_376588493: FileFormat = FileFormat {
    id: 376_588_493,
    source_type: SourceType::Iana,
    name: "rpki-ghostbusters",
    extensions: &[],
    media_types: &["application/rpki-ghostbusters"],
    signatures: &[],
    related_formats: &[],
};
