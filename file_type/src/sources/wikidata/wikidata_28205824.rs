use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205824: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_824,
        source_type: SourceType::Wikidata,
        name: "CgBI",
        extensions: &["png"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
