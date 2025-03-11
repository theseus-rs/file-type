use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854507: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_507,
        source_type: SourceType::Wikidata,
        name: "PAQ4 compressed archive",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x51, 0x34, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
