use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120861718: FileType = FileType {
    file_format: &FileFormat {
        id: 120_861_718,
        source_type: SourceType::Wikidata,
        name: "MyDVD Project",
        extensions: &["dvd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
