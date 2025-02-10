use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853894: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_894,
        source_type: SourceType::Wikidata,
        name: "Affix file",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x45, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
