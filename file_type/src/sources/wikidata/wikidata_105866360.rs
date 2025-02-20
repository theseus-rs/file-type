use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866360: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_360,
        source_type: SourceType::Wikidata,
        name: "Boeing Calc WorkPad (v3.x)",
        extensions: &["pad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x4C, 0x43, 0x33, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
