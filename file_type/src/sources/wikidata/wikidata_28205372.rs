use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205372: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_372,
        source_type: SourceType::Wikidata,
        name: "Kodak TIFF",
        extensions: &["tif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
