use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1170351643: FileFormat = FileFormat {
    id: 1_170_351_643,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.listcmd",
    extensions: &[],
    media_types: &["application/vnd.uplanet.listcmd"],
    signatures: &[],
    related_formats: &[],
};
