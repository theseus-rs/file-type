use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123202980: FileType = FileType {
    file_format: &FileFormat {
        id: 123_202_980,
        source_type: SourceType::Wikidata,
        name: "Microsoft DV-AVI Video File",
        extensions: &["dv-avi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
