use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856489: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_489,
        source_type: SourceType::Wikidata,
        name: "Hyper File database",
        extensions: &["fic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x53, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
