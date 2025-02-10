use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975862: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_862,
        source_type: SourceType::Wikidata,
        name: "OOGL Bezier Surface BBP",
        extensions: &["bbp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
