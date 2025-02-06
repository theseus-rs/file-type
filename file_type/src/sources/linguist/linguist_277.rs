use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_277: FileFormat = FileFormat {
    id: 277,
    source_type: SourceType::Linguist,
    name: "Papyrus",
    extensions: &["psc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
