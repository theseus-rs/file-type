use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592709: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_709,
        source_type: SourceType::Wikidata,
        name: "MuseScore format",
        extensions: &["mscz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
