use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3241319755: FileFormat = FileFormat {
    id: 3_241_319_755,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
