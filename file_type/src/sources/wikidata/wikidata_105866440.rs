use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866440: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_440,
        source_type: SourceType::Wikidata,
        name: "PA-RISC 1.1 object code (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x10, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
