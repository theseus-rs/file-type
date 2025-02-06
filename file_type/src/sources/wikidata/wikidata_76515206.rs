use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76515206: FileFormat = FileFormat {
    id: 76_515_206,
    source_type: SourceType::Wikidata,
    name: "Lotus 123 SQZ! compressed Worksheet",
    extensions: &["wk!"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
