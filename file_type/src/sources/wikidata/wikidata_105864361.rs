use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864361: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_361,
        source_type: SourceType::Wikidata,
        name: "GraphiCode Programmable Device Format",
        extensions: &["pdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAF, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
