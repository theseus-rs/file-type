use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859120: FileFormat = FileFormat {
    id: 105_859_120,
    puid: "wikidata/105859120",
    name: "PM XV bitmap (alt.endianess)",
    extensions: &["pm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x45, 0x49, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
