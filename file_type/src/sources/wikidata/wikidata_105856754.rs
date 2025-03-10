use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856754: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_754,
        source_type: SourceType::Wikidata,
        name: "Unisig - Uniform signature (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDC, 0xDC, 0x0D, 0x0A, 0x1A, 0x0A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
