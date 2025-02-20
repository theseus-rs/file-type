use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27824065: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_065,
        source_type: SourceType::Wikidata,
        name: "Internet Archive ARC, version 1.1",
        extensions: &["arc"],
        media_types: &["application/x-internet-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
