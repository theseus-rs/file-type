use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5427382: FileType = FileType {
    file_format: &FileFormat {
        id: 5_427_382,
        source_type: SourceType::Wikidata,
        name: "FXML",
        extensions: &["fxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
