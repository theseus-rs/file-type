use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862307: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_307,
        source_type: SourceType::Wikidata,
        name: "Melody Maker song (v3.x)",
        extensions: &["mm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEC, 0x4D, 0x4D, 0x76, 0x33, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
