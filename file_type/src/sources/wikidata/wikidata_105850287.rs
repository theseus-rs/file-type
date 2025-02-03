use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850287: FileFormat = FileFormat {
    id: 105_850_287,
    source_type: SourceType::Wikidata,
    name: "CryEngine Project (v5)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
