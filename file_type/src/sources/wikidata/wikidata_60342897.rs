use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60342897: FileType = FileType {
    file_format: &FileFormat {
        id: 60_342_897,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint Show",
        extensions: &["ppsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
