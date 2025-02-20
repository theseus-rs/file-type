use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117192597: FileType = FileType {
    file_format: &FileFormat {
        id: 117_192_597,
        source_type: SourceType::Wikidata,
        name: "Acrobat TouchUp Image",
        extensions: &["ai", "pdf", "pdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
