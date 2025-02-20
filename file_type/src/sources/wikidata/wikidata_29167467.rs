use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
