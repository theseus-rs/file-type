use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859948: FileFormat = FileFormat {
    id: 105_859_948,
    puid: "wikidata/105859948",
    name: "SplashID vID",
    extensions: &["vid"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x70, 0x6C, 0x61, 0x73, 0x68, 0x49, 0x44, 0x20, 0x76, 0x49, 0x44, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x20, 0x2D, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
