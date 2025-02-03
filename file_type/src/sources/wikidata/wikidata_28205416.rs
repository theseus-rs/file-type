use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205416: FileFormat = FileFormat {
    id: 28_205_416,
    source_type: SourceType::Wikidata,
    name: "Nikon Capture Image Dust Off File",
    extensions: &["ndr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
