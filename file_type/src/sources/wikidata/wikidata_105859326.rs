use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859326: FileFormat = FileFormat {
    id: 105_859_326,
    puid: "wikidata/105859326",
    name: "Qmage encoded data",
    extensions: &["qmg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
