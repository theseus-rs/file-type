use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860018: FileFormat = FileFormat {
    id: 105_860_018,
    puid: "wikidata/105860018",
    name: "V-Ray Material (XML)",
    extensions: &["vrmat"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x76, 0x69, 0x73, 0x6D, 0x61, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
