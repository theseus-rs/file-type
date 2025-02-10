use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66662117: FileType = FileType {
    file_format: &FileFormat {
        id: 66_662_117,
        source_type: SourceType::Wikidata,
        name: "Lotus Ami Pro Styles",
        extensions: &["sty"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
