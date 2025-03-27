use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133567482: FileType = FileType {
    file_format: &FileFormat {
        id: 133_567_482,
        source_type: SourceType::Wikidata,
        name: "Dune Image",
        extensions: &["aii"],
        media_types: &["image/x-dune"],
        signatures: &[],
        related_formats: &[],
    },
};
