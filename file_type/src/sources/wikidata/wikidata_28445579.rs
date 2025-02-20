use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445579: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_579,
        source_type: SourceType::Wikidata,
        name: "AcqKnowledge File",
        extensions: &["acq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
