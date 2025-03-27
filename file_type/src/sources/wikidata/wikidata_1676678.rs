use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1676678: FileType = FileType {
    file_format: &FileFormat {
        id: 1_676_678,
        source_type: SourceType::Wikidata,
        name: "JPEG Network Graphics",
        extensions: &["jng"],
        media_types: &["image/jng", "image/x-jng"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8B, 0x4A, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
