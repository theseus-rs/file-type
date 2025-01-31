use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851357: FileFormat = FileFormat {
    id: 105_851_357,
    puid: "wikidata/105851357",
    name: "TGF chemical test (UNIX)",
    extensions: &["tgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
