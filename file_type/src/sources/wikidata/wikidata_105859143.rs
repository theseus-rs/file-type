use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859143: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_143,
        source_type: SourceType::Wikidata,
        name: "Total Commander button Bar config",
        extensions: &["bar"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x75, 0x74, 0x74, 0x6F, 0x6E, 0x62, 0x61, 0x72, 0x5D, 0x0D,
                        0x0A, 0x42, 0x75, 0x74, 0x74, 0x6F, 0x6E, 0x63, 0x6F, 0x75, 0x6E, 0x74,
                        0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
