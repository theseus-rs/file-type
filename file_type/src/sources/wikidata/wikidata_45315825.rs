use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45315825: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_825,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand file format, version 10",
        extensions: &["fh10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
