use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_46120375: FileType = FileType {
    file_format: &FileFormat {
        id: 46_120_375,
        source_type: SourceType::Wikidata,
        name: "Lotus Notes Database file format, version 4",
        extensions: &["ns4", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
