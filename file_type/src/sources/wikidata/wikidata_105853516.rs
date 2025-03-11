use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_516,
        source_type: SourceType::Wikidata,
        name: "ZLIB compressed data (var. 1)",
        extensions: &[],
        media_types: &["application/zlib"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
