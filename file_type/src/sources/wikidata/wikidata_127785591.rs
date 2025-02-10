use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127785591: FileType = FileType {
    file_format: &FileFormat {
        id: 127_785_591,
        source_type: SourceType::Wikidata,
        name: "MetaPost PostScript file",
        extensions: &["mps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
