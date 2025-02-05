use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1358713128: FileFormat = FileFormat {
    id: 1_358_713_128,
    source_type: SourceType::Iana,
    name: "vnd.qualcomm.brew-app-res",
    extensions: &[],
    media_types: &["application/vnd.qualcomm.brew-app-res"],
    signatures: &[],
    related_formats: &[],
};
