use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50375253: FileType = FileType {
    file_format: &FileFormat {
        id: 50_375_253,
        source_type: SourceType::Wikidata,
        name: "Extensible Metadata Platform Format",
        extensions: &["xmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
