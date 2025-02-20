use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131626493: FileType = FileType {
    file_format: &FileFormat {
        id: 131_626_493,
        source_type: SourceType::Wikidata,
        name: "Tabulat data file format",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
