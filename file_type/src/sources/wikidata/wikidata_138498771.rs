use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138498771: FileType = FileType {
    file_format: &FileFormat {
        id: 138_498_771,
        source_type: SourceType::Wikidata,
        name: "Kodak DCS RAW Image 3",
        extensions: &["tif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
