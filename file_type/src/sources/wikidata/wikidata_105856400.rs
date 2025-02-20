use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_400,
        source_type: SourceType::Wikidata,
        name: "Cakewalk Music project",
        extensions: &["wrk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x4B, 0x45, 0x57, 0x41, 0x4C, 0x4B, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
