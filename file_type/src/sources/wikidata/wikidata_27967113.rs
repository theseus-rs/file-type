use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967113: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_113,
        source_type: SourceType::Wikidata,
        name: "AProSys module",
        extensions: &["aps"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x44, 0x52, 0x56, 0x50, 0x41, 0x43, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
