use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122168574: FileType = FileType {
    file_format: &FileFormat {
        id: 122_168_574,
        source_type: SourceType::Wikidata,
        name: "Old Security Explorer Project",
        extensions: &["nsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
