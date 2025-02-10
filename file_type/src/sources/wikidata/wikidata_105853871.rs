use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853871: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_871,
        source_type: SourceType::Wikidata,
        name: "Aperture Version info",
        extensions: &["apversion"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30, 0xDF, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
