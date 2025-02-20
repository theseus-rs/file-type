use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975670: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_670,
        source_type: SourceType::Wikidata,
        name: "Cyberspace Description Format",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
