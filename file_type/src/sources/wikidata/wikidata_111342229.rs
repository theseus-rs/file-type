use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342229: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_229,
        source_type: SourceType::Wikidata,
        name: "Sounder sound file",
        extensions: &["sndr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
