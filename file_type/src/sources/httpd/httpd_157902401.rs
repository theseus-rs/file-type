use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_157902401: FileFormat = FileFormat {
    id: 157_902_401,
    source_type: SourceType::Httpd,
    name: "fujixerox edmics mmr",
    extensions: &["mmr"],
    media_types: &["image/vnd.fujixerox.edmics-mmr"],
    signatures: &[],
    related_formats: &[],
};
