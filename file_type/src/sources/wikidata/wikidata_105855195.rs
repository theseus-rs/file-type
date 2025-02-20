use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855195: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_195,
        source_type: SourceType::Wikidata,
        name: "Flash Project",
        extensions: &["flp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x66, 0x6C, 0x61, 0x73, 0x68, 0x5F, 0x70, 0x72,
                        0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
