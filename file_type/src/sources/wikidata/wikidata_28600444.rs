use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600444: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_444,
        source_type: SourceType::Wikidata,
        name: "crontab file format",
        extensions: &[],
        media_types: &["text/x-crontab"],
        signatures: &[],
        related_formats: &[],
    },
};
