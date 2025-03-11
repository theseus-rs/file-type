use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855037: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_037,
        source_type: SourceType::Wikidata,
        name: "Interplay ACMP audio",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6E, 0x74, 0x65, 0x72, 0x70, 0x6C, 0x61, 0x79, 0x20, 0x41, 0x43,
                        0x4D, 0x50, 0x20, 0x44, 0x61, 0x74, 0x61, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
