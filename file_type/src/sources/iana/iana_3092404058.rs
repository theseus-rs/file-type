use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3092404058: FileType = FileType {
    file_format: &FileFormat {
        id: 3_092_404_058,
        source_type: SourceType::Iana,
        name: "voucher-jws+json",
        extensions: &[],
        media_types: &["application/voucher-jws+json"],
        signatures: &[],
        related_formats: &[],
    },
};
