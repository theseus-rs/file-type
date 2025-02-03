use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_101072836: FileFormat = FileFormat {
    id: 101_072_836,
    source_type: SourceType::Iana,
    name: "vnd.syncml.ds.notification",
    extensions: &[],
    media_types: &["application/vnd.syncml.ds.notification"],
    internal_signatures: &[],
    related_formats: &[],
};
