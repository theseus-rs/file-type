use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2576353381: FileType = FileType {
    file_format: &FileFormat {
        id: 2_576_353_381,
        source_type: SourceType::Iana,
        name: "vnd.gnu.taler.exchange+json",
        extensions: &[],
        media_types: &["application/vnd.gnu.taler.exchange+json"],
        signatures: &[],
        related_formats: &[],
    },
};
