use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_325264537: FileFormat = FileFormat {
    id: 325_264_537,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-conf+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-conf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
