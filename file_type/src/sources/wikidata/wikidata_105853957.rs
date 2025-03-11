use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853957: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_957,
        source_type: SourceType::Wikidata,
        name: "AUSREP Sailing Plan (SP)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x55, 0x53, 0x52, 0x45, 0x50, 0x2F, 0x53, 0x50, 0x2F, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
