use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50419770: FileType = FileType {
    file_format: &FileFormat {
        id: 50_419_770,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Graphics Metafile",
        extensions: &["wpg"],
        media_types: &["image/x-wpg"],
        signatures: &[],
        related_formats: &[],
    },
};
