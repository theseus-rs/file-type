use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58007215: FileType = FileType {
    file_format: &FileFormat {
        id: 58_007_215,
        source_type: SourceType::Wikidata,
        name: "Visual Basic File",
        extensions: &["vb"],
        media_types: &["text/x-vba", "text/x-vbnet"],
        signatures: &[],
        related_formats: &[],
    },
};
