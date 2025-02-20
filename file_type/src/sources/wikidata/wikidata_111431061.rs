use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111431061: FileType = FileType {
    file_format: &FileFormat {
        id: 111_431_061,
        source_type: SourceType::Wikidata,
        name: "C# source code file",
        extensions: &["cs"],
        media_types: &["text/x-csharp"],
        signatures: &[],
        related_formats: &[],
    },
};
