use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111283245: FileFormat = FileFormat {
    id: 111_283_245,
    source_type: SourceType::Wikidata,
    name: "Floating point raw 64-bit IEEE data",
    extensions: &["f64"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
