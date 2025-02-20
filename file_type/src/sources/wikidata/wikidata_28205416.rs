use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205416: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_416,
        source_type: SourceType::Wikidata,
        name: "Nikon Capture Image Dust Off File",
        extensions: &["ndr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
