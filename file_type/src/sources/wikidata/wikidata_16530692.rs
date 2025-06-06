use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_16530692: FileType = FileType {
    file_format: &FileFormat {
        id: 16_530_692,
        source_type: SourceType::Wikidata,
        name: "BED",
        extensions: &["bed"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x72, 0x6F, 0x77, 0x73, 0x65, 0x72, 0x20, 0x70, 0x6F, 0x73, 0x69,
                        0x74, 0x69, 0x6F, 0x6E, 0x20, 0x63, 0x68, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
