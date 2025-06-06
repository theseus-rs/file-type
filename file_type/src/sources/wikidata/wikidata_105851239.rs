use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851239: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_239,
        source_type: SourceType::Wikidata,
        name: "Grigon texture",
        extensions: &["tex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x72, 0x69, 0x67, 0x6F, 0x6E, 0x20, 0x54, 0x65, 0x78, 0x74, 0x75,
                        0x72, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
