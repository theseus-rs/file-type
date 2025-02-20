use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771324: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_324,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
