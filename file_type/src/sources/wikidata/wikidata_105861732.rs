use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_732,
        source_type: SourceType::Wikidata,
        name: "J.River Media Center plugin",
        extensions: &["mjp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
