use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4836515: FileType = FileType {
    file_format: &FileFormat {
        id: 4_836_515,
        source_type: SourceType::Wikidata,
        name: "BSAVE",
        extensions: &["bsv", "cgx", "pic", "scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
