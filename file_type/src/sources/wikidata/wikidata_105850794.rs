use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850794: FileFormat = FileFormat {
    id: 105_850_794,
    puid: "wikidata/105850794",
    name: "Katorzer music",
    extensions: &["kat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x61, 0x54, 0x6F, 0x72, 0x5A, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
