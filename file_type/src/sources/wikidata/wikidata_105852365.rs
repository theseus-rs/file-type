use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852365: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_365,
        source_type: SourceType::Wikidata,
        name: "Sereal serialized data (v3-4)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0xF3, 0x72, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
