use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_242,
        source_type: SourceType::Wikidata,
        name: "ScriptMagic archive",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4D, 0x32, 0x4D, 0x50, 0x58, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
