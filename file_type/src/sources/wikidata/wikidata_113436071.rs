use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113436071: FileType = FileType {
    file_format: &FileFormat {
        id: 113_436_071,
        source_type: SourceType::Wikidata,
        name: "INTREPID Standard Information File",
        extensions: &["isi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
