use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1340693: FileType = FileType {
    file_format: &FileFormat {
        id: 1_340_693,
        source_type: SourceType::Wikidata,
        name: "RealMedia Variable Bitrate",
        extensions: &["rmvb"],
        media_types: &["application/vnd.rn-realmedia-vbr"],
        signatures: &[],
        related_formats: &[],
    },
};
