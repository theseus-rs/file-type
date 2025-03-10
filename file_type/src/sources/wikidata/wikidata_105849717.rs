use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849717: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_717,
        source_type: SourceType::Wikidata,
        name: "Call Of Duty weapons data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x45, 0x41, 0x50, 0x4F, 0x4E, 0x46, 0x49, 0x4C, 0x45, 0x5C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
