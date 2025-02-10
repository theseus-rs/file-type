use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_307107080: FileType = FileType {
    file_format: &FileFormat {
        id: 307_107_080,
        source_type: SourceType::Iana,
        name: "prs.sid",
        extensions: &[],
        media_types: &["audio/prs.sid"],
        signatures: &[],
        related_formats: &[],
    },
};
