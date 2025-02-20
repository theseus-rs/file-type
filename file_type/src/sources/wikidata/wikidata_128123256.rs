use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128123256: FileType = FileType {
    file_format: &FileFormat {
        id: 128_123_256,
        source_type: SourceType::Wikidata,
        name: "Stylus file format",
        extensions: &["styl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
