use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850659: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_659,
        source_type: SourceType::Wikidata,
        name: "Hondata K-Manager Calibration data",
        extensions: &["kal"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x0D, 0x0A, 0x3B, 0x20, 0x48, 0x6F, 0x6E, 0x64, 0x61, 0x74, 0x61,
                        0x20, 0x4B, 0x2D, 0x4D, 0x61, 0x6E, 0x61, 0x67, 0x65, 0x72, 0x20, 0x43,
                        0x61, 0x6C, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
