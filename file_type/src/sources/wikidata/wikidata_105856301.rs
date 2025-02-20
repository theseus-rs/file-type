use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856301: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_301,
        source_type: SourceType::Wikidata,
        name: "Avogadro Drawlog",
        extensions: &["drawlog"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x41, 0x76, 0x6F, 0x67, 0x61, 0x64, 0x72, 0x6F, 0x20, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
