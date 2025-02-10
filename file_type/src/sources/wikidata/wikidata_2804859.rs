use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2804859: FileType = FileType {
    file_format: &FileFormat {
        id: 2_804_859,
        source_type: SourceType::Wikidata,
        name: "VDA-FS",
        extensions: &["vda", "vdafs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
