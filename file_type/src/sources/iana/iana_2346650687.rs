use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2346650687: FileFormat = FileFormat {
    id: 2_346_650_687,
    source_type: SourceType::Iana,
    name: "vnd.japannet-verification",
    extensions: &[],
    media_types: &["application/vnd.japannet-verification"],
    internal_signatures: &[],
    related_formats: &[],
};
