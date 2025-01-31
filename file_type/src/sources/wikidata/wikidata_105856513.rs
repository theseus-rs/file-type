use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856513: FileFormat = FileFormat {
    id: 105_856_513,
    puid: "wikidata/105856513",
    name: "Xara graphics",
    extensions: &["wix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4C, 0x41, 0x52, 0x45, 0x54, 0x45, 0x58, 0x54, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
