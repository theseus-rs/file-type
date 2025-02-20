use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855579: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_579,
        source_type: SourceType::Wikidata,
        name: "OPAM package info",
        extensions: &["opam"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x70, 0x61, 0x6D, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3A, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
