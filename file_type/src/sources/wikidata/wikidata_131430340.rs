use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131430340: FileType = FileType {
    file_format: &FileFormat {
        id: 131_430_340,
        source_type: SourceType::Wikidata,
        name: "X10 file format",
        extensions: &["x10"],
        media_types: &["text/x-x10"],
        signatures: &[],
        related_formats: &[],
    },
};
