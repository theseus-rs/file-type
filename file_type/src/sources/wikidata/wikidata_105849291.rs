use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849291: FileFormat = FileFormat {
    id: 105_849_291,
    puid: "wikidata/105849291",
    name: "Yabause movie capture",
    extensions: &["ymv"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x0D, 0x0A, 0x65, 0x6D,
                    0x75, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x39, 0x0D, 0x0A, 0x72,
                    0x65, 0x72, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x43, 0x6F, 0x75, 0x6E, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
