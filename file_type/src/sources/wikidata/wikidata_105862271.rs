use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862271: FileFormat = FileFormat {
    id: 105_862_271,
    source_type: SourceType::Wikidata,
    name: "The Bone Shaker Architect music",
    extensions: &["bsa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x42, 0x53, 0x41, 0x30, 0x2E, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
