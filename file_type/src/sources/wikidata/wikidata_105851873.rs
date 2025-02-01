use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851873: FileFormat = FileFormat {
    id: 105_851_873,
    puid: "wikidata/105851873",
    name: "GNU TeXmacs Scheme",
    extensions: &["stm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x28, 0x54, 0x65,
                    0x58, 0x6D, 0x61, 0x63, 0x73, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
