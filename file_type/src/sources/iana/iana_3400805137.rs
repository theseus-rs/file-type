use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3400805137: FileFormat = FileFormat {
    id: 3_400_805_137,
    source_type: SourceType::Iana,
    name: "vnd.osa.netdeploy",
    extensions: &[],
    media_types: &["application/vnd.osa.netdeploy"],
    signatures: &[],
    related_formats: &[],
};
