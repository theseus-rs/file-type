use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28009492: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_492,
        source_type: SourceType::Wikidata,
        name: "Warcraft II PUD",
        extensions: &["pud"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
