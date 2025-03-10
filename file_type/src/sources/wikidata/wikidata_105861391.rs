use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_391,
        source_type: SourceType::Wikidata,
        name: "Linden wearable definition",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4C, 0x57, 0x65, 0x61, 0x72, 0x61, 0x62, 0x6C, 0x65, 0x20, 0x76,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
