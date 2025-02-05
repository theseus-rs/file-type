use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2269607461: FileFormat = FileFormat {
    id: 2_269_607_461,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.ddd",
    extensions: &[],
    media_types: &["application/vnd.fujixerox.ddd"],
    signatures: &[],
    related_formats: &[],
};
