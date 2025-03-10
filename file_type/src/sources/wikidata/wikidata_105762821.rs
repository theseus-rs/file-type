use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762821: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_821,
        source_type: SourceType::Wikidata,
        name: "XBOX executable",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x42, 0x45, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
