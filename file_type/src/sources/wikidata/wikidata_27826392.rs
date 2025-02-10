use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826392: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_392,
        source_type: SourceType::Wikidata,
        name: "Proxy Unrestricted Access Image",
        extensions: &["uai"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
