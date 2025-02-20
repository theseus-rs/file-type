use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128770247: FileType = FileType {
    file_format: &FileFormat {
        id: 128_770_247,
        source_type: SourceType::Wikidata,
        name: "CFEngine3 file format",
        extensions: &["cf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
