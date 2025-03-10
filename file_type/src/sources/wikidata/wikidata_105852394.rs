use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852394: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_394,
        source_type: SourceType::Wikidata,
        name: "Source Code Control System history data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x68])],
                },
            }],
        }],
        related_formats: &[],
    },
};
