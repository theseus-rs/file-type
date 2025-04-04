use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855102: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_102,
        source_type: SourceType::Wikidata,
        name: "IRCAM Sound Format audio (VAX be)",
        extensions: &["sf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0xA3, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
