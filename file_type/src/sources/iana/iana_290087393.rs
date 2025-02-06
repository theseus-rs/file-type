use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_290087393: FileFormat = FileFormat {
    id: 290_087_393,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.docuworks",
    extensions: &[],
    media_types: &["application/vnd.fujixerox.docuworks"],
    signatures: &[],
    related_formats: &[],
};
