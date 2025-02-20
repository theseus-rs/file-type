use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63339218: FileType = FileType {
    file_format: &FileFormat {
        id: 63_339_218,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 4.5",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
