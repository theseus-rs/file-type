use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855139: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_139,
        source_type: SourceType::Wikidata,
        name: "Micro Magic Forms in Flight Sequence",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x6D, 0x61, 0x67, 0x69, 0x63, 0x20,
                        0x73, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
