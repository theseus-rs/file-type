use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2341024932: FileType = FileType {
    file_format: &FileFormat {
        id: 2_341_024_932,
        source_type: SourceType::Iana,
        name: "vnd.senx.warpscript",
        extensions: &[],
        media_types: &["text/vnd.senx.warpscript"],
        signatures: &[],
        related_formats: &[],
    },
};
