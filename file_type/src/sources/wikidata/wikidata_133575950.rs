use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133575950: FileType = FileType {
    file_format: &FileFormat {
        id: 133_575_950,
        source_type: SourceType::Wikidata,
        name: "GFA Artist",
        extensions: &["art"],
        media_types: &["image/x-gfa-artist"],
        signatures: &[],
        related_formats: &[],
    },
};
