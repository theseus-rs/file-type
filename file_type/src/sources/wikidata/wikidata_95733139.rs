use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95733139: FileType = FileType {
    file_format: &FileFormat {
        id: 95_733_139,
        source_type: SourceType::Wikidata,
        name: "RealAudio version 3",
        extensions: &["ra"],
        media_types: &["audio/vnd.rn-realaudio"],
        signatures: &[],
        related_formats: &[],
    },
};
