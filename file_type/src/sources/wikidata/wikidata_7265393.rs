use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7265393: FileType = FileType {
    file_format: &FileFormat {
        id: 7_265_393,
        source_type: SourceType::Wikidata,
        name: "QCP",
        extensions: &["qcp"],
        media_types: &["audio/qcelp", "audio/vnd.qcelp"],
        signatures: &[],
        related_formats: &[],
    },
};
