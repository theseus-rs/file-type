use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95733736: FileType = FileType {
    file_format: &FileFormat {
        id: 95_733_736,
        source_type: SourceType::Wikidata,
        name: "RealAudio Metafile",
        extensions: &["ram"],
        media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
        signatures: &[],
        related_formats: &[],
    },
};
