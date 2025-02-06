use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3776309532: FileFormat = FileFormat {
    id: 3_776_309_532,
    source_type: SourceType::Iana,
    name: "vnd.cns.inf2",
    extensions: &[],
    media_types: &["image/vnd.cns.inf2"],
    signatures: &[],
    related_formats: &[],
};
