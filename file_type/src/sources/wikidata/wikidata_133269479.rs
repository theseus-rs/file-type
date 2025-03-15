use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133269479: FileType = FileType {
    file_format: &FileFormat {
        id: 133_269_479,
        source_type: SourceType::Wikidata,
        name: "Plextalk Project File (imtt)",
        extensions: &["imtt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
