use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63166360: FileType = FileType {
    file_format: &FileFormat {
        id: 63_166_360,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Binder Wizard for Windows, version 97-2003",
        extensions: &["obz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
