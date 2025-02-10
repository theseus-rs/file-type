use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125823475: FileType = FileType {
    file_format: &FileFormat {
        id: 125_823_475,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help merged query index file",
        extensions: &["hxq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
