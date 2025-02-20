use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111271738: FileType = FileType {
    file_format: &FileFormat {
        id: 111_271_738,
        source_type: SourceType::Wikidata,
        name: "Delusion/XTracker sample format",
        extensions: &["dsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
