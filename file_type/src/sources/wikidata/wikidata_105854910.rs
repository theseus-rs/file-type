use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854910: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_910,
        source_type: SourceType::Wikidata,
        name: "IRCAM Sound Format audio (NeXT)",
        extensions: &["sf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0xA3, 0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
