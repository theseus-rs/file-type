use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860479: FileFormat = FileFormat {
    id: 105_860_479,
    puid: "wikidata/105860479",
    name: "Eureka/Mercury Report",
    extensions: &["rpt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCD, 0xCD, 0xCD, 0xCD, 0xCD, 0x20, 0x50, 0x72, 0x6F, 0x62, 0x6C, 0x65, 0x6D,
                    0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
