use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_210,
        source_type: SourceType::Wikidata,
        name: "NIST Sphere waveform audio",
        extensions: &["nist", "sph"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x49, 0x53, 0x54, 0x5F, 0x31, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
