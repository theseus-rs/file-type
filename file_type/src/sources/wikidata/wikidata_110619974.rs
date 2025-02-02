use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110619974: FileFormat = FileFormat {
    id: 110_619_974,
    source_type: SourceType::Wikidata,
    name: "Adobe Atmosphere world (.atmo)",
    extensions: &["3da", "aer", "atmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
