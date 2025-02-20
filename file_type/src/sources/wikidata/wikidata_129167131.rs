use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129167131: FileType = FileType {
    file_format: &FileFormat {
        id: 129_167_131,
        source_type: SourceType::Wikidata,
        name: "Evoque file format",
        extensions: &["evoque"],
        media_types: &["application/x-evoque"],
        signatures: &[],
        related_formats: &[],
    },
};
