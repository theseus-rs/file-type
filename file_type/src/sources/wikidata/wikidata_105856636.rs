use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856636: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_636,
        source_type: SourceType::Wikidata,
        name: "Windows Error Report",
        extensions: &["wer"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00, 0x6F, 0x00,
                        0x6E, 0x00, 0x3D, 0x00, 0x31, 0x00, 0x0D, 0x00, 0x0A, 0x00, 0x45, 0x00,
                        0x76, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x74, 0x00, 0x54, 0x00, 0x79, 0x00,
                        0x70, 0x00, 0x65, 0x00, 0x3D, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
