use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131412989: FileType = FileType {
    file_format: &FileFormat {
        id: 131_412_989,
        source_type: SourceType::Wikidata,
        name: "VisualProlog file format",
        extensions: &["cl", "i", "pack", "ph", "pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
