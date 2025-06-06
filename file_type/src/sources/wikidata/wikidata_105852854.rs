use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105852854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_854,
        source_type: SourceType::Wikidata,
        name: "Sublime Text Project",
        extensions: &["sublime-project"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
