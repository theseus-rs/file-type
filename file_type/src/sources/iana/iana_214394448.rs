use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_214394448: FileFormat = FileFormat {
    id: 214_394_448,
    source_type: SourceType::Iana,
    name: "vnd.wap.wmlscript",
    extensions: &[],
    media_types: &["text/vnd.wap.wmlscript"],
    internal_signatures: &[],
    related_formats: &[],
};
