use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853487: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_487,
        source_type: SourceType::Wikidata,
        name: "Inno Setup data",
        extensions: &["bin", "dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x6C, 0x62, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
