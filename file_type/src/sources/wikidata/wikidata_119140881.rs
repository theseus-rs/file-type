use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119140881: FileType = FileType {
    file_format: &FileFormat {
        id: 119_140_881,
        source_type: SourceType::Wikidata,
        name: "FreeHand Template 10",
        extensions: &["ft10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
