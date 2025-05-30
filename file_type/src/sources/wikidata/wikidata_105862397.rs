use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862397: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_397,
        source_type: SourceType::Wikidata,
        name: "Multimedia Builder Data",
        extensions: &["mbd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0x4D, 0x4D, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
