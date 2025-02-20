use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63165558: FileType = FileType {
    file_format: &FileFormat {
        id: 63_165_558,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Binder Template for Windows, version 97-2003",
        extensions: &["obt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
