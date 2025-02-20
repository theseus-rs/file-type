use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131684737: FileType = FileType {
    file_format: &FileFormat {
        id: 131_684_737,
        source_type: SourceType::Wikidata,
        name: "Movie.BYU file format",
        extensions: &["g"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
