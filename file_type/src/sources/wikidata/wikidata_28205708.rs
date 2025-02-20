use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205708: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_708,
        source_type: SourceType::Wikidata,
        name: "Applixware Bitmap",
        extensions: &["im"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
