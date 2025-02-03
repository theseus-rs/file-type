use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1516953976: FileFormat = FileFormat {
    id: 1_516_953_976,
    source_type: SourceType::Iana,
    name: "vnd.ecowin.seriesupdate",
    extensions: &[],
    media_types: &["application/vnd.ecowin.seriesupdate"],
    internal_signatures: &[],
    related_formats: &[],
};
