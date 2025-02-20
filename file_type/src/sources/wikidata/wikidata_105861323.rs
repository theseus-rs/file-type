use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_323,
        source_type: SourceType::Wikidata,
        name: "Lineage II Replay",
        extensions: &["l2r"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x17, 0x4C, 0x69, 0x6E, 0x65, 0x61, 0x67, 0x65, 0x49, 0x49, 0x20, 0x52,
                        0x65, 0x70, 0x6C, 0x61, 0x79, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
