use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855341: FileFormat = FileFormat {
    id: 105_855_341,
    puid: "wikidata/105855341",
    name: "XPS FixedPage object",
    extensions: &["fpage"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x46, 0x69, 0x78, 0x65, 0x64, 0x50, 0x61, 0x67, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
