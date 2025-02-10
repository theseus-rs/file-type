use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854081: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_081,
        source_type: SourceType::Wikidata,
        name: "Yamaha TX-16W samples audio",
        extensions: &["txw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4D, 0x38, 0x39, 0x35, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
