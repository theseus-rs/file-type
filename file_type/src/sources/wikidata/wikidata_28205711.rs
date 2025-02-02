use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205711: FileFormat = FileFormat {
    id: 28_205_711,
    source_type: SourceType::Wikidata,
    name: "STW",
    extensions: &["stw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
