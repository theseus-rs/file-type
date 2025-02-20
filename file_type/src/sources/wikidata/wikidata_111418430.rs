use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111418430: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_430,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Collection File",
        extensions: &["collection"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
