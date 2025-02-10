use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861681: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_681,
        source_type: SourceType::Wikidata,
        name: "Mednafen/PCEjin/VBjin movie capture",
        extensions: &["mc2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
