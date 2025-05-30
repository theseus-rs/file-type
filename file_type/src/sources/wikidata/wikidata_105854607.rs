use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854607: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_607,
        source_type: SourceType::Wikidata,
        name: "Arkos Tracker binary music (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x54, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
