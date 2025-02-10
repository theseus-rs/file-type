use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63165182: FileType = FileType {
    file_format: &FileFormat {
        id: 63_165_182,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Binder Wizard for Windows",
        extensions: &["obz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
