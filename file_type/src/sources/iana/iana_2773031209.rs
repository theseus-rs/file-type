use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2773031209: FileFormat = FileFormat {
    id: 2_773_031_209,
    source_type: SourceType::Iana,
    name: "vnd.afpc.afplinedata",
    extensions: &[],
    media_types: &["application/vnd.afpc.afplinedata"],
    internal_signatures: &[],
    related_formats: &[],
};
