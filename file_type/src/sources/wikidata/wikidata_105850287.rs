use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850287: FileFormat = FileFormat {
    id: 105_850_287,
    source_type: SourceType::Wikidata,
    name: "CryEngine Project (v5)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
