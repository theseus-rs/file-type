use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238819: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_819,
        source_type: SourceType::Wikidata,
        name: "Movie Magic Scheduling Export",
        extensions: &["sex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x53, 0x49, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
