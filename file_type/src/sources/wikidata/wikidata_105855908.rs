use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855908: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_908,
        source_type: SourceType::Wikidata,
        name: "Ovation Pro document",
        extensions: &["dpd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x76, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x50, 0x72, 0x6F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
