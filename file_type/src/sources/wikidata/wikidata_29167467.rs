use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167467: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_467,
        source_type: SourceType::Wikidata,
        name: "OME-XML",
        extensions: &["ome.xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
