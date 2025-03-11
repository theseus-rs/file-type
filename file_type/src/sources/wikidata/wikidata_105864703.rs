use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864703: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_703,
        source_type: SourceType::Wikidata,
        name: "ProvideX Indexed data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x56, 0x58, 0x49, 0x4E, 0x44, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
