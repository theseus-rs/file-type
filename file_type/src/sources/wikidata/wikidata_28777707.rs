use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28777707: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_707,
        source_type: SourceType::Wikidata,
        name: "mzML",
        extensions: &["mxml", "mzML", "mzml"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
