use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2269607461: FileType = FileType {
    file_format: &FileFormat {
        id: 2_269_607_461,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.ddd",
        extensions: &[],
        media_types: &["application/vnd.fujixerox.ddd"],
        signatures: &[],
        related_formats: &[],
    },
};
