use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333099: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_099,
        source_type: SourceType::Wikidata,
        name: "Korg PA1X/PA800/PA2X samples",
        extensions: &["pcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
