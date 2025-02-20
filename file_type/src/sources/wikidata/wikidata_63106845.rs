use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63106845: FileType = FileType {
    file_format: &FileFormat {
        id: 63_106_845,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Binder Template for Windows",
        extensions: &["obt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
