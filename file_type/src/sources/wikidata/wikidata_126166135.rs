use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126166135: FileType = FileType {
    file_format: &FileFormat {
        id: 126_166_135,
        source_type: SourceType::Wikidata,
        name: "Compressed MusicXML 3.1+",
        extensions: &["mxl"],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
