use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826346: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_346,
        source_type: SourceType::Wikidata,
        name: "Reduced resolution dataset file",
        extensions: &["rrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
