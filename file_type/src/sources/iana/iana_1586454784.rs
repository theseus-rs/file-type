use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1586454784: FileFormat = FileFormat {
    id: 1_586_454_784,
    source_type: SourceType::Iana,
    name: "vnd.vmx.cvsd",
    extensions: &[],
    media_types: &["audio/vnd.vmx.cvsd"],
    internal_signatures: &[],
    related_formats: &[],
};
