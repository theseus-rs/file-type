use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113324648: FileType = FileType {
    file_format: &FileFormat {
        id: 113_324_648,
        source_type: SourceType::Wikidata,
        name: "Pixlr Layered Image",
        extensions: &["pxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
