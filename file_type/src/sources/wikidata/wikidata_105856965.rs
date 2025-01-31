use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856965: FileFormat = FileFormat {
    id: 105_856_965,
    puid: "wikidata/105856965",
    name: "Stata Graph",
    extensions: &["gph"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x61, 0x74, 0x61, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x4D, 0x3A, 0x30,
                    0x30, 0x30, 0x30, 0x31, 0x3A, 0x30, 0x31, 0x30, 0x30, 0x30, 0x3A, 0x4C, 0x69,
                    0x76, 0x65, 0x47, 0x50, 0x48, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
