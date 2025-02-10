use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55739293: FileType = FileType {
    file_format: &FileFormat {
        id: 55_739_293,
        source_type: SourceType::Wikidata,
        name: "CRAM file format, version 1",
        extensions: &["cram"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
