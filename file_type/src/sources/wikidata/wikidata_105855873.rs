use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_873,
        source_type: SourceType::Wikidata,
        name: "Eudemons Online game data",
        extensions: &["dnp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x61, 0x77, 0x6E, 0x50, 0x61, 0x63, 0x6B, 0x2E, 0x54, 0x71, 0x44,
                        0x69, 0x67, 0x69, 0x74, 0x61, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
