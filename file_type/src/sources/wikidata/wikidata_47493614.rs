use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47493614: FileType = FileType {
    file_format: &FileFormat {
        id: 47_493_614,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CS3",
        extensions: &["ind", "indd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
