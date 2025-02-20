use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125823450: FileType = FileType {
    file_format: &FileFormat {
        id: 125_823_450,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help Data file",
        extensions: &["hxr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
