use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3000375204: FileType = FileType {
    file_format: &FileFormat {
        id: 3_000_375_204,
        source_type: SourceType::Iana,
        name: "vnd.oma.poc.detailed-progress-report+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.poc.detailed-progress-report+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
