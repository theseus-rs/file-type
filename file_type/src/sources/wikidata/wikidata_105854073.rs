use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854073: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_073,
        source_type: SourceType::Wikidata,
        name: "GenCAM object model",
        extensions: &["a3p"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
