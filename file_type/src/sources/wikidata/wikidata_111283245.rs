use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111283245: FileFormat = FileFormat {
    id: 111_283_245,
    source_type: SourceType::Wikidata,
    name: "Floating point raw 64-bit IEEE data",
    extensions: &["f64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
