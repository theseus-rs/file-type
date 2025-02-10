use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975638: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_638,
        source_type: SourceType::Wikidata,
        name: "Parasolid",
        extensions: &["x_t"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
