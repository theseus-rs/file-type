use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125822813: FileType = FileType {
    file_format: &FileFormat {
        id: 125_822_813,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help Combined Full-text Search file",
        extensions: &["chq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
