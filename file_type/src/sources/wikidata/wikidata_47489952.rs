use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47489952: FileType = FileType {
    file_format: &FileFormat {
        id: 47_489_952,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 3",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
