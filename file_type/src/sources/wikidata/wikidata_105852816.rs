use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_816,
        source_type: SourceType::Wikidata,
        name: "IDA Signatures",
        extensions: &["sig"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x41, 0x53, 0x47, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
