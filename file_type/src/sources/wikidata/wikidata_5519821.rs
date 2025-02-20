use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5519821: FileType = FileType {
    file_format: &FileFormat {
        id: 5_519_821,
        source_type: SourceType::Wikidata,
        name: "Game Boy Sound",
        extensions: &["gbs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x42, 0x53, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
