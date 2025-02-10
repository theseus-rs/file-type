use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853689: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_689,
        source_type: SourceType::Wikidata,
        name: "LZBW1 compressed data",
        extensions: &["lz1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD2, 0xCC, 0x30, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
