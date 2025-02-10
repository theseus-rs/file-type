use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_159747167: FileType = FileType {
    file_format: &FileFormat {
        id: 159_747_167,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.pic-bw-small",
        extensions: &[],
        media_types: &["application/vnd.3gpp.pic-bw-small"],
        signatures: &[],
        related_formats: &[],
    },
};
