use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1398477285: FileFormat = FileFormat {
    id: 1_398_477_285,
    source_type: SourceType::Iana,
    name: "vnd.firemonkeys.cloudcell",
    extensions: &[],
    media_types: &["application/vnd.firemonkeys.cloudcell"],
    internal_signatures: &[],
    related_formats: &[],
};
