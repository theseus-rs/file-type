use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127803507: FileType = FileType {
    file_format: &FileFormat {
        id: 127_803_507,
        source_type: SourceType::Wikidata,
        name: "Mojo source code file",
        extensions: &["mojo", "🔥"],
        media_types: &["application/x-mojo", "text/x-mojo"],
        signatures: &[],
        related_formats: &[],
    },
};
