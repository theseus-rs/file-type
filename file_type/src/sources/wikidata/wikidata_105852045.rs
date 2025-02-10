use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852045: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_045,
        source_type: SourceType::Wikidata,
        name: "Quake II Save game",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7C, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
