use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855583: FileFormat = FileFormat {
    id: 105_855_583,
    puid: "wikidata/105855583",
    name: "World Construction Set Object",
    extensions: &["obj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x53, 0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x3F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
