use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860376: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_376,
        source_type: SourceType::Wikidata,
        name: "Rebel spreadsheet (v2)",
        extensions: &["rb2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7E, 0x7E, 0x53, 0x76, 0x32, 0x7E, 0x7E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
