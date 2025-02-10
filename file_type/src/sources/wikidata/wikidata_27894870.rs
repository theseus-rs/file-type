use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27894870: FileType = FileType {
    file_format: &FileFormat {
        id: 27_894_870,
        source_type: SourceType::Wikidata,
        name: "Windows Media Audio Redirector",
        extensions: &["wax"],
        media_types: &["audio/x-ms-wax"],
        signatures: &[],
        related_formats: &[],
    },
};
