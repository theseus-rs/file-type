use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4450230: FileType = FileType {
    file_format: &FileFormat {
        id: 4_450_230,
        source_type: SourceType::Wikidata,
        name: "GCA",
        extensions: &[],
        media_types: &["application/x-gca-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
