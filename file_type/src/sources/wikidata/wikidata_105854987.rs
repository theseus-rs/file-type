use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854987: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_987,
        source_type: SourceType::Wikidata,
        name: "IRCAM Sound Format audio (SUN native)",
        extensions: &["sf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0xA3, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
