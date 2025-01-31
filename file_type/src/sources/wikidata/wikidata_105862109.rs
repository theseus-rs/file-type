use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862109: FileFormat = FileFormat {
    id: 105_862_109,
    puid: "wikidata/105862109",
    name: "PuavoHard Intro Music Composer module (v3)",
    extensions: &["phpimc"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0xFF, 0x0B, 0x70, 0x00, 0x68, 0x00, 0x70, 0x00, 0x69, 0x00, 0x6D,
                    0x00, 0x63, 0x00, 0x20, 0x00, 0x76, 0x00, 0x33, 0x00, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
