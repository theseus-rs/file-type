use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856941: FileFormat = FileFormat {
    id: 105_856_941,
    puid: "wikidata/105856941",
    name: "Mind Games - Chinese Chess saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x48, 0x45, 0x53, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
