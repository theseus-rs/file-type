use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3326048280: FileFormat = FileFormat {
    id: 3_326_048_280,
    source_type: SourceType::Iana,
    name: "vnd.multiad.creator.cif",
    extensions: &[],
    media_types: &["application/vnd.multiad.creator.cif"],
    signatures: &[],
    related_formats: &[],
};
