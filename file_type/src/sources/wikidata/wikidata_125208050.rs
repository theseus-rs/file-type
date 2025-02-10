use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125208050: FileType = FileType {
    file_format: &FileFormat {
        id: 125_208_050,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project XML",
        extensions: &["mspxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
