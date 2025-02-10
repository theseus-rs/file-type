use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979270: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_270,
        source_type: SourceType::Wikidata,
        name: "TheDraw Save File",
        extensions: &["td"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
