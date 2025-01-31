use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854944: FileFormat = FileFormat {
    id: 105_854_944,
    puid: "wikidata/105854944",
    name: "Active Tutor data",
    extensions: &["arf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0C, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x54, 0x75, 0x74, 0x6F, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
