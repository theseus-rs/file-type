use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864532: FileFormat = FileFormat {
    id: 105_864_532,
    puid: "wikidata/105864532",
    name: "Brown Bag Word Processor Printer control ruler",
    extensions: &["prt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6F, 0x6E, 0x74,
                    0x72, 0x6F, 0x6C, 0x20, 0x72, 0x75, 0x6C, 0x65, 0x72, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
