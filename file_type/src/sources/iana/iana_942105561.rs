use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_942105561: FileType = FileType {
    file_format: &FileFormat {
        id: 942_105_561,
        source_type: SourceType::Iana,
        name: "rpki-manifest",
        extensions: &[],
        media_types: &["application/rpki-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
