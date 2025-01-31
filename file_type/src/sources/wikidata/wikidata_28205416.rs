use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205416: FileFormat = FileFormat {
    id: 28_205_416,
    puid: "wikidata/28205416",
    name: "Nikon Capture Image Dust Off File",
    extensions: &["ndr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
