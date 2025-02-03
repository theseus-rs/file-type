use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2803185257: FileFormat = FileFormat {
    id: 2_803_185_257,
    source_type: SourceType::Iana,
    name: "cdmi-queue",
    extensions: &[],
    media_types: &["application/cdmi-queue"],
    internal_signatures: &[],
    related_formats: &[],
};
