use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205679: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_679,
        source_type: SourceType::Wikidata,
        name: "Amber ARR Bitmap Image",
        extensions: &["arr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
