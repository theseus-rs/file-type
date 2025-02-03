use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_731233819: FileFormat = FileFormat {
    id: 731_233_819,
    source_type: SourceType::Linguist,
    name: "NWScript",
    extensions: &["nss"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
