use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851389: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_389,
        source_type: SourceType::Wikidata,
        name: "Truevision3D Model",
        extensions: &["tvm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x56, 0x4D, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
