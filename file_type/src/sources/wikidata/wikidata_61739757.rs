use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61739757: FileType = FileType {
    file_format: &FileFormat {
        id: 61_739_757,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 5",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
