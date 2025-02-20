use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858178: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_178,
        source_type: SourceType::Wikidata,
        name: "Europa Universalis IV saved game",
        extensions: &["eu4"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x55, 0x34, 0x74, 0x78, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
