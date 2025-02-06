use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205416: FileFormat = FileFormat {
    id: 28_205_416,
    source_type: SourceType::Wikidata,
    name: "Nikon Capture Image Dust Off File",
    extensions: &["ndr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
