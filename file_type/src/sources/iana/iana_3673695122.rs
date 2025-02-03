use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3673695122: FileFormat = FileFormat {
    id: 3_673_695_122,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.docuworks.container",
    extensions: &[],
    media_types: &["application/vnd.fujixerox.docuworks.container"],
    internal_signatures: &[],
    related_formats: &[],
};
