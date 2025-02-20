use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857097: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_097,
        source_type: SourceType::Wikidata,
        name: "GPS Tuner map calibration data",
        extensions: &["gmi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x70, 0x20, 0x43, 0x61, 0x6C, 0x69, 0x62, 0x72, 0x61, 0x74,
                        0x69, 0x6F, 0x6E, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
