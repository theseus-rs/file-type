use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3400961159: FileType = FileType {
    file_format: &FileFormat {
        id: 3_400_961_159,
        source_type: SourceType::Iana,
        name: "vnd.openblox.game-binary",
        extensions: &[],
        media_types: &["application/vnd.openblox.game-binary"],
        signatures: &[],
        related_formats: &[],
    },
};
