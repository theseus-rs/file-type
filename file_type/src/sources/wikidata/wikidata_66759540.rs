use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66759540: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_540,
        source_type: SourceType::Wikidata,
        name: "Excel 97-2003 Template",
        extensions: &["xlt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
