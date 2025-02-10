use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009592: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_592,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Label File format",
        extensions: &["lbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
