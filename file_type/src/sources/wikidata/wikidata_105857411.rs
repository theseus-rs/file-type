use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857411: FileFormat = FileFormat {
    id: 105_857_411,
    source_type: SourceType::Wikidata,
    name: "BlackBerry JDE Application Project",
    extensions: &["jdp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x20, 0x52, 0x49, 0x4D, 0x20, 0x4A, 0x61, 0x76, 0x61, 0x20, 0x44,
                    0x65, 0x76, 0x65, 0x6C, 0x6F, 0x70, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x45, 0x6E,
                    0x76, 0x69, 0x72, 0x6F, 0x6E, 0x6D, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
