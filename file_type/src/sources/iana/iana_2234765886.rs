use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2234765886: FileFormat = FileFormat {
    id: 2_234_765_886,
    source_type: SourceType::Iana,
    name: "encrypted",
    extensions: &[],
    media_types: &["multipart/encrypted"],
    internal_signatures: &[],
    related_formats: &[],
};
