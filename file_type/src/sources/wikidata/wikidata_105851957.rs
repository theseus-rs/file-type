use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851957: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_957,
        source_type: SourceType::Wikidata,
        name: "Snes9x snapshot",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x73, 0x6E, 0x65, 0x73, 0x39, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
