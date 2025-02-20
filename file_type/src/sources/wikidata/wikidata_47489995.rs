use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47489995: FileType = FileType {
    file_format: &FileFormat {
        id: 47_489_995,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 6",
        extensions: &["fm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
