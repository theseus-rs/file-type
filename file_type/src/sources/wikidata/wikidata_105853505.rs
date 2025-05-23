use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_505,
        source_type: SourceType::Wikidata,
        name: "ZLIB compressed data (var. 4)",
        extensions: &[],
        media_types: &["application/zlib"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0xDA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
