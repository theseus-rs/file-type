use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126951711: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_711,
        source_type: SourceType::Wikidata,
        name: "NetRexx source code file",
        extensions: &["nrx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
