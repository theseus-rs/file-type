use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3969471112: FileType = FileType {
    file_format: &FileFormat {
        id: 3_969_471_112,
        source_type: SourceType::Iana,
        name: "vnd.oma.cab-user-prefs+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.cab-user-prefs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
