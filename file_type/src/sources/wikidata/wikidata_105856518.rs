use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_518,
        source_type: SourceType::Wikidata,
        name: "WINDEV data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
