use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206216: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_216,
        source_type: SourceType::Wikidata,
        name: "GrayPaint",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
