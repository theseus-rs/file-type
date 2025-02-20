use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114237015: FileType = FileType {
    file_format: &FileFormat {
        id: 114_237_015,
        source_type: SourceType::Wikidata,
        name: "Dialog Script",
        extensions: &["dlg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
