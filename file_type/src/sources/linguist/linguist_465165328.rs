use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_465165328: FileFormat = FileFormat {
    id: 465_165_328,
    source_type: SourceType::Linguist,
    name: "JetBrains MPS",
    extensions: &["mpl", "mps", "msd"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
