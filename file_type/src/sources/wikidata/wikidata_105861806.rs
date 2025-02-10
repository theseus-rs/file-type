use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861806: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_806,
        source_type: SourceType::Wikidata,
        name: "Stimulsoft Reports report (JSON)",
        extensions: &["mrt"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x0D, 0x0A, 0x20, 0x20, 0x22, 0x52, 0x65, 0x70, 0x6F, 0x72, 0x74,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x3A, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
