use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856462: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_462,
        source_type: SourceType::Wikidata,
        name: "PGE World Map",
        extensions: &["wldx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
