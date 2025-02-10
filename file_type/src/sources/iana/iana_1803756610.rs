use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1803756610: FileType = FileType {
    file_format: &FileFormat {
        id: 1_803_756_610,
        source_type: SourceType::Iana,
        name: "vnd.globalplatform.card-content-mgt-response",
        extensions: &[],
        media_types: &["application/vnd.globalplatform.card-content-mgt-response"],
        signatures: &[],
        related_formats: &[],
    },
};
