use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_898,
        source_type: SourceType::Wikidata,
        name: "Magic Lantern DIFF animation",
        extensions: &["diff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x49, 0x46, 0x46, 0x48, 0xA8, 0x9A, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
