use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5310: FileType = FileType {
    file_format: &FileFormat {
        id: 5_310,
        source_type: SourceType::Wikidata,
        name: "LaTeX",
        extensions: &["tex"],
        media_types: &["application/x-latex"],
        signatures: &[],
        related_formats: &[],
    },
};
