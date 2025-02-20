use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_191,
        source_type: SourceType::Wikidata,
        name: "Supersonic Software game data archive",
        extensions: &["piz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x5A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
