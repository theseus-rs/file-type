use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_451700185: FileFormat = FileFormat {
    id: 451_700_185,
    source_type: SourceType::Linguist,
    name: "AIDL",
    extensions: &["aidl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
