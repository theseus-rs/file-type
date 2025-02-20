use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1346528962: FileType = FileType {
    file_format: &FileFormat {
        id: 1_346_528_962,
        source_type: SourceType::Iana,
        name: "vnd.oai.workflows+yaml",
        extensions: &[],
        media_types: &["application/vnd.oai.workflows+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
