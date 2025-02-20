use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473308: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_308,
        source_type: SourceType::Wikidata,
        name: "CADRG Frame File",
        extensions: &["ccz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
