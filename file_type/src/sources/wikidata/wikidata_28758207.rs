use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28758207: FileFormat = FileFormat {
    id: 28_758_207,
    source_type: SourceType::Wikidata,
    name: "Adaptive Prediction Trees",
    extensions: &["apt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
