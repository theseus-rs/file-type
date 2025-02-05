use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758207: FileFormat = FileFormat {
    id: 28_758_207,
    source_type: SourceType::Wikidata,
    name: "Adaptive Prediction Trees",
    extensions: &["apt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
