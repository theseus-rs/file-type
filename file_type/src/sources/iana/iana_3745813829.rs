use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3745813829: FileType = FileType {
    file_format: &FileFormat {
        id: 3_745_813_829,
        source_type: SourceType::Iana,
        name: "alto-endpointpropparams+json",
        extensions: &[],
        media_types: &["application/alto-endpointpropparams+json"],
        signatures: &[],
        related_formats: &[],
    },
};
