use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_220079965: FileFormat = FileFormat {
    id: 220_079_965,
    source_type: SourceType::Iana,
    name: "vnd.ms-wmdrm.meter-resp",
    extensions: &[],
    media_types: &["application/vnd.ms-wmdrm.meter-resp"],
    internal_signatures: &[],
    related_formats: &[],
};
