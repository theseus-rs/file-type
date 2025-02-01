use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854627: FileFormat = FileFormat {
    id: 105_854_627,
    puid: "wikidata/105854627",
    name: "Artemis Presents! document (v2.0)",
    extensions: &["apr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x54, 0x52, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
