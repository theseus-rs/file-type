use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859491: FileFormat = FileFormat {
    id: 105_859_491,
    puid: "wikidata/105859491",
    name: "Altera Quartus IP",
    extensions: &["qip"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x65, 0x74, 0x5F, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x5F, 0x61, 0x73,
                    0x73, 0x69, 0x67, 0x6E, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
