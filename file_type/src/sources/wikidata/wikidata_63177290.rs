use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63177290: FileType = FileType {
    file_format: &FileFormat {
        id: 63_177_290,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet for Macintosh, version 3",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
