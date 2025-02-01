use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206620: FileFormat = FileFormat {
    id: 28_206_620,
    puid: "wikidata/28206620",
    name: "Microsoft Paint, version 2",
    extensions: &["msp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x69, 0x6E, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
