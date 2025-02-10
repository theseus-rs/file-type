use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854508: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_508,
        source_type: SourceType::Wikidata,
        name: "Recorded voice audio",
        extensions: &["zvr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x56, 0x52, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
