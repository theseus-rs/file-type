use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137735890: FileType = FileType {
    file_format: &FileFormat {
        id: 137_735_890,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate theme file",
        extensions: &["cptm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
