use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206390: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_390,
        source_type: SourceType::Wikidata,
        name: "IRIS CMYK Front End Processor CT",
        extensions: &["ct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
