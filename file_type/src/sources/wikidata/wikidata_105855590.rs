use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855590: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_590,
        source_type: SourceType::Wikidata,
        name: "LEONARD'S Sketch drawing",
        extensions: &["ogf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x00, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x70, 0x61, 0x63, 0x65,
                        0x06, 0x00, 0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
