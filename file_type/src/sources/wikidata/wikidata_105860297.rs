use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860297: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_297,
        source_type: SourceType::Wikidata,
        name: "romfs image",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x72, 0x6F, 0x6D, 0x31, 0x66, 0x73, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
