use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853574: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_574,
        source_type: SourceType::Wikidata,
        name: "ZX-Live Snapshot",
        extensions: &["zls"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x2D, 0x4C, 0x69, 0x76, 0x65, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
