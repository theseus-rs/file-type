use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50223749: FileType = FileType {
    file_format: &FileFormat {
        id: 50_223_749,
        source_type: SourceType::Wikidata,
        name: "compressed MusicXML",
        extensions: &["mxl"],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
