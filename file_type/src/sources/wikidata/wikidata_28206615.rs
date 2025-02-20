use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206615: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_615,
        source_type: SourceType::Wikidata,
        name: "Microsoft Paint, version 1",
        extensions: &["msp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x61, 0x6E, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
