use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207202: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_202,
        source_type: SourceType::Wikidata,
        name: "QuickTime Image Format",
        extensions: &["qif", "qtif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
