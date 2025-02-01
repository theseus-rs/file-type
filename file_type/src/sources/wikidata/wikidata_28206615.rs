use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206615: FileFormat = FileFormat {
    id: 28_206_615,
    puid: "wikidata/28206615",
    name: "Microsoft Paint, version 1",
    extensions: &["msp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x61, 0x6E, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
