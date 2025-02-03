use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_338748118: FileFormat = FileFormat {
    id: 338_748_118,
    source_type: SourceType::Iana,
    name: "vnd.informedcontrol.rms+xml",
    extensions: &[],
    media_types: &["application/vnd.informedcontrol.rms+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
