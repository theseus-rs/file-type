use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855058: FileFormat = FileFormat {
    id: 105_855_058,
    source_type: SourceType::Wikidata,
    name: "WinAsks Editor Analysis (v2.00)",
    extensions: &["anl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x17, 0x00, 0x57, 0x69, 0x6E, 0x41, 0x73, 0x6B, 0x73, 0x20, 0x45, 0x64, 0x69,
                    0x74, 0x6F, 0x72, 0x20, 0x41, 0x6E, 0x61, 0x6C, 0x79, 0x73, 0x69, 0x73, 0x04,
                    0x00, 0x32, 0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
