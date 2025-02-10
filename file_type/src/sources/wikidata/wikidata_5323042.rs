use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_5323042: FileType = FileType {
    file_format: &FileFormat {
        id: 5_323_042,
        source_type: SourceType::Wikidata,
        name: "EGG",
        extensions: &["egg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x47, 0x47, 0x41, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
