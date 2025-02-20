use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3075306099: FileType = FileType {
    file_format: &FileFormat {
        id: 3_075_306_099,
        source_type: SourceType::Iana,
        name: "EDI-consent",
        extensions: &[],
        media_types: &["application/EDI-consent"],
        signatures: &[],
        related_formats: &[],
    },
};
