use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853206: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_206,
        source_type: SourceType::Wikidata,
        name: "Flashback Sprite",
        extensions: &["spr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x50, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
