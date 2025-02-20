use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859933: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_933,
        source_type: SourceType::Wikidata,
        name: "Volition Package game archive data",
        extensions: &["vp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x50, 0x56, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
