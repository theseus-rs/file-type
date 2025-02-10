use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975882: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_882,
        source_type: SourceType::Wikidata,
        name: "SOLIDWORKS Assembly",
        extensions: &["sldasm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
