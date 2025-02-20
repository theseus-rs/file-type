use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117857310: FileType = FileType {
    file_format: &FileFormat {
        id: 117_857_310,
        source_type: SourceType::Wikidata,
        name: "IBM Storyboard PIC file",
        extensions: &["sbp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
