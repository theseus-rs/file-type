use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122676986: FileType = FileType {
    file_format: &FileFormat {
        id: 122_676_986,
        source_type: SourceType::Wikidata,
        name: "CMX Corel Clipart",
        extensions: &["cmx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
