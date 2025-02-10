use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858872: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_872,
        source_type: SourceType::Wikidata,
        name: "HBasic source code",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x42, 0x41, 0x53, 0x64, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
