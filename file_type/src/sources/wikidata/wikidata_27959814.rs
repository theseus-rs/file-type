use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959814: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_814,
        source_type: SourceType::Wikidata,
        name: "Ableton Meta Sound",
        extensions: &["ams"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
