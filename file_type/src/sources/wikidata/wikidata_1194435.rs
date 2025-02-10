use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1194435: FileType = FileType {
    file_format: &FileFormat {
        id: 1_194_435,
        source_type: SourceType::Wikidata,
        name: "MPEG-2 transport stream",
        extensions: &["ts", "tsa", "tsv"],
        media_types: &["video/mp2t"],
        signatures: &[],
        related_formats: &[],
    },
};
