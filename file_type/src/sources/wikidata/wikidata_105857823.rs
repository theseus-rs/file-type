use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_823,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine paperdoll (v1)",
        extensions: &["plt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4C, 0x54, 0x20, 0x56, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
