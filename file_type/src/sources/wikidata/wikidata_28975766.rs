use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975766: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_766,
        source_type: SourceType::Wikidata,
        name: "DMO format",
        extensions: &["dmo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
