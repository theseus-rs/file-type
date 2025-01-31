use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858891: FileFormat = FileFormat {
    id: 105_858_891,
    puid: "wikidata/105858891",
    name: "Microsoft Store download package",
    extensions: &["box"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x00, 0xC5, 0xB0])],
            },
        }],
    }],
    related_formats: &[],
};
