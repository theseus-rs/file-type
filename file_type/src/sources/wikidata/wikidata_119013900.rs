use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119013900: FileType = FileType {
    file_format: &FileFormat {
        id: 119_013_900,
        source_type: SourceType::Wikidata,
        name: "Binspector grammar",
        extensions: &["bfft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
