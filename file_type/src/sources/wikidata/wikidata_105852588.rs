use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852588: FileFormat = FileFormat {
    id: 105_852_588,
    source_type: SourceType::Wikidata,
    name: "3D Master Scene",
    extensions: &["scene"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x46, 0x53, 0x36, 0x36, 0x30, 0x33, 0x32, 0x31, 0x76, 0x30, 0x30, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
