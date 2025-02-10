use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_46120116: FileType = FileType {
    file_format: &FileFormat {
        id: 46_120_116,
        source_type: SourceType::Wikidata,
        name: "Lotus Notes Database file format, version 3",
        extensions: &["ns3", "nsf"],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
