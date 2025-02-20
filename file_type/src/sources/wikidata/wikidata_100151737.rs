use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100151737: FileType = FileType {
    file_format: &FileFormat {
        id: 100_151_737,
        source_type: SourceType::Wikidata,
        name: "Muvee autoProducer Project File",
        extensions: &["mve"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
