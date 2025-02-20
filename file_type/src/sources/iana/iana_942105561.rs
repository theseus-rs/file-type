use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
