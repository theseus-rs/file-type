use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4212993566: FileType = FileType {
    file_format: &FileFormat {
        id: 4_212_993_566,
        source_type: SourceType::Iana,
        name: "vnd.etsi.tsl.der",
        extensions: &[],
        media_types: &["application/vnd.etsi.tsl.der"],
        signatures: &[],
        related_formats: &[],
    },
};
