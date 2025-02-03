use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1934041989: FileFormat = FileFormat {
    id: 1_934_041_989,
    source_type: SourceType::Iana,
    name: "vnd.ms-xpsdocument",
    extensions: &[],
    media_types: &["application/vnd.ms-xpsdocument"],
    internal_signatures: &[],
    related_formats: &[],
};
