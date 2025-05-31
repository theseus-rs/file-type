use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857269: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_269,
        source_type: SourceType::Wikidata,
        name: "Horizon EDA Project",
        extensions: &["hprj"],
        media_types: &["text/json"],
        signatures: &[],
        related_formats: &[],
    },
};
