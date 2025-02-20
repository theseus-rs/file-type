use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112653501: FileType = FileType {
    file_format: &FileFormat {
        id: 112_653_501,
        source_type: SourceType::Wikidata,
        name: "Professional Draw 1.0 backup file",
        extensions: &["pd~"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
