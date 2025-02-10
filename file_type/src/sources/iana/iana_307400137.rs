use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_307400137: FileType = FileType {
    file_format: &FileFormat {
        id: 307_400_137,
        source_type: SourceType::Iana,
        name: "vnd.igloader",
        extensions: &[],
        media_types: &["application/vnd.igloader"],
        signatures: &[],
        related_formats: &[],
    },
};
