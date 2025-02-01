use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854076: FileFormat = FileFormat {
    id: 105_854_076,
    puid: "wikidata/105854076",
    name: "athtune script",
    extensions: &["athtune"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x73, 0x79, 0x6E, 0x74, 0x68, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
