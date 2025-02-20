use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130542831: FileType = FileType {
    file_format: &FileFormat {
        id: 130_542_831,
        source_type: SourceType::Wikidata,
        name: "Pug file format",
        extensions: &["jade", "pug"],
        media_types: &["text/x-jade", "text/x-pug"],
        signatures: &[],
        related_formats: &[],
    },
};
