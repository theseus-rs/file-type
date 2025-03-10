use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850507: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_507,
        source_type: SourceType::Wikidata,
        name: "Code Cruncher 3 packed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF3, 0x4B, 0x53, 0x41, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
