use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_97033393: FileType = FileType {
    file_format: &FileFormat {
        id: 97_033_393,
        source_type: SourceType::Wikidata,
        name: "Magick Scripting Language",
        extensions: &["msl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
