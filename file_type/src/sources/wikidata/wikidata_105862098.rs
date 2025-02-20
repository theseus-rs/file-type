use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_098,
        source_type: SourceType::Wikidata,
        name: "Miva Compiled script",
        extensions: &["mvc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x69, 0x76, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
