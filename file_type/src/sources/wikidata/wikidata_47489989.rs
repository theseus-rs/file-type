use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47489989: FileType = FileType {
    file_format: &FileFormat {
        id: 47_489_989,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 5.5",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
