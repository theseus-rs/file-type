use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854387: FileFormat = FileFormat {
    id: 105_854_387,
    source_type: SourceType::Wikidata,
    name: "Calamus ASCII Translation Table",
    extensions: &["att"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20, 0x41, 0x54, 0x54, 0x20, 0x23,
                    0x20, 0x41, 0x54, 0x54, 0x20, 0x3D, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20,
                    0x54, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F,
                    0x6E, 0x20, 0x54, 0x61, 0x62, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
