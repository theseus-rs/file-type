use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_46119878: FileType = FileType {
    file_format: &FileFormat {
        id: 46_119_878,
        source_type: SourceType::Wikidata,
        name: "Lotus Notes Database file format, version 2",
        extensions: &["ns2", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
