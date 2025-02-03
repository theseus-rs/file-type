use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600223: FileFormat = FileFormat {
    id: 28_600_223,
    source_type: SourceType::Wikidata,
    name: "APT",
    extensions: &["apt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
