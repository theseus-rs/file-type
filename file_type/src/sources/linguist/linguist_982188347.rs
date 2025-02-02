use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_982188347: FileFormat = FileFormat {
    id: 982_188_347,
    source_type: SourceType::Linguist,
    name: "BibTeX",
    extensions: &["bib", "bibtex"],
    media_types: &["text/x-stex"],
    internal_signatures: &[],
    related_formats: &[],
};
