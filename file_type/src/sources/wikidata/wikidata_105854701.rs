use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854701: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_701,
        source_type: SourceType::Wikidata,
        name: "IRCAM Sound Format audio (MIPS SGI)",
        extensions: &["sf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x03, 0xA3, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
