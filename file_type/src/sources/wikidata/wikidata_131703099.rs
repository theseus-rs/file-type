use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131703099: FileType = FileType {
    file_format: &FileFormat {
        id: 131_703_099,
        source_type: SourceType::Wikidata,
        name: "VERA output file",
        extensions: &["h5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
