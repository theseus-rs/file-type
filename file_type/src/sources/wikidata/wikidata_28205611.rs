use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205611: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_611,
        source_type: SourceType::Wikidata,
        name: "RIPscrip version 1 Hot Icon",
        extensions: &["hot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
