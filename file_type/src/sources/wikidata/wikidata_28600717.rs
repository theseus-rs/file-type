use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600717: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_717,
        source_type: SourceType::Wikidata,
        name: "DrawIt",
        extensions: &["drawit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
