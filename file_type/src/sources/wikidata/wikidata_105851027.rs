use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851027: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_027,
        source_type: SourceType::Wikidata,
        name: "T81 EightyOne tape image",
        extensions: &["t81"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4F, 0x38, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
