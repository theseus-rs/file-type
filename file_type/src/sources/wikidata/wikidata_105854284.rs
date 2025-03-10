use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854284: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_284,
        source_type: SourceType::Wikidata,
        name: "zisofs compressed format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0xE4, 0x53, 0x96, 0xC9, 0xDB, 0xD6, 0x07,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
