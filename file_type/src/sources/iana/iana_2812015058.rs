use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2812015058: FileType = FileType {
    file_format: &FileFormat {
        id: 2_812_015_058,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.listcmd-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.listcmd-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
