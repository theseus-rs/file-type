use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967115: FileFormat = FileFormat {
    id: 27_967_115,
    source_type: SourceType::Wikidata,
    name: "Art of Noise module",
    extensions: &["aon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
