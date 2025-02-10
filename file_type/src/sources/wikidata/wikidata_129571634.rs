use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129571634: FileType = FileType {
    file_format: &FileFormat {
        id: 129_571_634,
        source_type: SourceType::Wikidata,
        name: "Hy source code file",
        extensions: &["hy"],
        media_types: &["application/x-hy", "text/x-hy"],
        signatures: &[],
        related_formats: &[],
    },
};
