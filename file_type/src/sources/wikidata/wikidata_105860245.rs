use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_245,
        source_type: SourceType::Wikidata,
        name: "Windows Registry Data (Ver. 4.0)",
        extensions: &["reg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x47, 0x45, 0x44, 0x49, 0x54, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
