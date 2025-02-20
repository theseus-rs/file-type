use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_92442998: FileType = FileType {
    file_format: &FileFormat {
        id: 92_442_998,
        source_type: SourceType::Wikidata,
        name: "Numpy compressed array format",
        extensions: &["npz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
