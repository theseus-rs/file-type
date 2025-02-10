use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_794,
        source_type: SourceType::Wikidata,
        name: "Exatron String Floppy virtual wafer image",
        extensions: &["esf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x53, 0x46, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
