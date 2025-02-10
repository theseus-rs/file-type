use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762701: FileFormat = FileFormat {
    id: 105_762_701,
    source_type: SourceType::Wikidata,
    name: "Draw.io raw XML diagram",
    extensions: &["drawio", "xml"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x78, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x41,
                    0x67, 0x65, 0x6E, 0x74, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
