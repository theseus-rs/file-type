use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551294: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_294,
        source_type: SourceType::Wikidata,
        name: "Adobe Color Table File",
        extensions: &["act"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
