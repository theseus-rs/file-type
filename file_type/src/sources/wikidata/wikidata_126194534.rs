use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126194534: FileType = FileType {
    file_format: &FileFormat {
        id: 126_194_534,
        source_type: SourceType::Wikidata,
        name: "MySQL View Definition Format",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
