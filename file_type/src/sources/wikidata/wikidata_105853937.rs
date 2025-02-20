use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853937: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_937,
        source_type: SourceType::Wikidata,
        name: "BSSC compressed data",
        extensions: &["bssc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x53, 0x53, 0x43, 0x5F, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
