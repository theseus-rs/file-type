use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131341382: FileType = FileType {
    file_format: &FileFormat {
        id: 131_341_382,
        source_type: SourceType::Wikidata,
        name: "Typst code",
        extensions: &["typ"],
        media_types: &["text/x-typst"],
        signatures: &[],
        related_formats: &[],
    },
};
