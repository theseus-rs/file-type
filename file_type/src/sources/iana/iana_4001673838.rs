use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4001673838: FileFormat = FileFormat {
    id: 4_001_673_838,
    source_type: SourceType::Iana,
    name: "vnd.efi.iso",
    extensions: &[],
    media_types: &["application/vnd.efi.iso"],
    internal_signatures: &[],
    related_formats: &[],
};
