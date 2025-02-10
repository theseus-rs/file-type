use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853847: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_847,
        source_type: SourceType::Wikidata,
        name: "ERI compressed archive",
        extensions: &["eri"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x69, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
