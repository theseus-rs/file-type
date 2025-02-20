use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3477565: FileType = FileType {
    file_format: &FileFormat {
        id: 3_477_565,
        source_type: SourceType::Wikidata,
        name: "Secure Digital Container",
        extensions: &["sdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
