use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_982188347: FileType = FileType {
    file_format: &FileFormat {
        id: 982_188_347,
        source_type: SourceType::Linguist,
        name: "BibTeX",
        extensions: &["bib", "bibtex"],
        media_types: &["text/x-stex"],
        signatures: &[],
        related_formats: &[],
    },
};
