use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_99761366: FileType = FileType {
    file_format: &FileFormat {
        id: 99_761_366,
        source_type: SourceType::Wikidata,
        name: "Canon Original RAW, version 3",
        extensions: &["cr3"],
        media_types: &["image/x-canon-cr3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x63, 0x72, 0x78, 0x20,
                        0x00, 0x00, 0x00, 0x01, 0x63, 0x72, 0x78, 0x20, 0x69, 0x73, 0x6F, 0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
