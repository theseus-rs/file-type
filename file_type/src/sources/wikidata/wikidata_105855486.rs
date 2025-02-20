use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855486: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_486,
        source_type: SourceType::Wikidata,
        name: "The Need For Speed Font",
        extensions: &["ffn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4E, 0x54, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
