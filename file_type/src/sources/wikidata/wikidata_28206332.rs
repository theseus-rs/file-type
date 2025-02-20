use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206332: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_332,
        source_type: SourceType::Wikidata,
        name: "Img Software Set Green Component",
        extensions: &["g"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
