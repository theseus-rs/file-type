use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853685: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_685,
        source_type: SourceType::Wikidata,
        name: "LZHAM compressed data",
        extensions: &["lzham"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x5A, 0x48, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
