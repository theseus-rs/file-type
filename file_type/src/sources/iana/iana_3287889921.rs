use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3287889921: FileFormat = FileFormat {
    id: 3_287_889_921,
    source_type: SourceType::Iana,
    name: "vnd.dolby.pl2z",
    extensions: &[],
    media_types: &["audio/vnd.dolby.pl2z"],
    signatures: &[],
    related_formats: &[],
};
