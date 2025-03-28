use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857432: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_432,
        source_type: SourceType::Wikidata,
        name: "ParaJVE ROM",
        extensions: &["jverom"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x56, 0x45, 0x52, 0x4F, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
