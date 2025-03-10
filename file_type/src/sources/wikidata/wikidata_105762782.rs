use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_782,
        source_type: SourceType::Wikidata,
        name: "BORGChat smiles",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x42, 0x4F, 0x52, 0x47, 0x53, 0x6D, 0x69, 0x6C, 0x65, 0x3E, 0x0D,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
