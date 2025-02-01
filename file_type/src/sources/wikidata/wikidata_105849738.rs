use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849738: FileFormat = FileFormat {
    id: 105_849_738,
    puid: "wikidata/105849738",
    name: "Microsoft Project 4.0 for DOS Calendar",
    extensions: &["cal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x43, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
