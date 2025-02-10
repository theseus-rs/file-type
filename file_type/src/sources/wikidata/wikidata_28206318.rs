use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206318: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_318,
        source_type: SourceType::Wikidata,
        name: "Img Software Set Colormapped Image",
        extensions: &["p"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
