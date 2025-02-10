use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55739507: FileType = FileType {
    file_format: &FileFormat {
        id: 55_739_507,
        source_type: SourceType::Wikidata,
        name: "Genbox Family History file format",
        extensions: &["GDB"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
