use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849308: FileFormat = FileFormat {
    id: 105_849_308,
    puid: "wikidata/105849308",
    name: "YGOPRO replay",
    extensions: &["yrp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x79, 0x72, 0x70, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
