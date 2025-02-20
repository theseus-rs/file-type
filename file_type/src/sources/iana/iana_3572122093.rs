use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3572122093: FileType = FileType {
    file_format: &FileFormat {
        id: 3_572_122_093,
        source_type: SourceType::Iana,
        name: "vnd.chipnuts.karaoke-mmd",
        extensions: &[],
        media_types: &["application/vnd.chipnuts.karaoke-mmd"],
        signatures: &[],
        related_formats: &[],
    },
};
