use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861108: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_108,
        source_type: SourceType::Wikidata,
        name: "LabVIEW project Library",
        extensions: &["lvlib"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x27, 0x31, 0x2E, 0x30, 0x27,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
