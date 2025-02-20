use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852841: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_841,
        source_type: SourceType::Wikidata,
        name: "PlayStation Sprite Editor project File",
        extensions: &["sdf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x4D, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
