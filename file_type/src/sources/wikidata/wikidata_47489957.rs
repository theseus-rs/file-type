use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47489957: FileType = FileType {
    file_format: &FileFormat {
        id: 47_489_957,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 4",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
