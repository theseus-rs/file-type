use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51800130: FileType = FileType {
    file_format: &FileFormat {
        id: 51_800_130,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Add-In",
        extensions: &["xla", "xll"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
