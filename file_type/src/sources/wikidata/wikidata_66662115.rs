use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66662115: FileType = FileType {
    file_format: &FileFormat {
        id: 66_662_115,
        source_type: SourceType::Wikidata,
        name: "Lotus Ami Pro Macro",
        extensions: &["smm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
