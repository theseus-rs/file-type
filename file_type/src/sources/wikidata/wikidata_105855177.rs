use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855177: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_177,
        source_type: SourceType::Wikidata,
        name: "FASA Interactive game data archive",
        extensions: &["fst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAF, 0xEC, 0xDD, 0xCA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
