use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45315902: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_902,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand file format, version 8",
        extensions: &["fh8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
