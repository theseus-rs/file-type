use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854306: FileFormat = FileFormat {
    id: 105_854_306,
    puid: "wikidata/105854306",
    name: "PR archiving tool archive",
    extensions: &["ar"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
