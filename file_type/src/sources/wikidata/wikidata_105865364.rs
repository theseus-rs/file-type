use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_364,
        source_type: SourceType::Wikidata,
        name: "Partclone image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x61, 0x72, 0x74, 0x63, 0x6C, 0x6F, 0x6E, 0x65, 0x2D, 0x69, 0x6D,
                        0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
