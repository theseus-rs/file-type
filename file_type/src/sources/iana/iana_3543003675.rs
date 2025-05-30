use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3543003675: FileType = FileType {
    file_format: &FileFormat {
        id: 3_543_003_675,
        source_type: SourceType::Iana,
        name: "cache-manifest",
        extensions: &[],
        media_types: &["text/cache-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
