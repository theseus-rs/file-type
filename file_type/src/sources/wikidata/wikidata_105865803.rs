use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865803: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_803,
        source_type: SourceType::Wikidata,
        name: "PCW LocoScript document (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4F, 0x59])],
                },
            }],
        }],
        related_formats: &[],
    },
};
