use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854820: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_820,
        source_type: SourceType::Wikidata,
        name: "Arahne yarn",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x59, 0x5F, 0x44, 0x45, 0x46, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
