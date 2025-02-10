use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865841: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_841,
        source_type: SourceType::Wikidata,
        name: "Power Tab Guitar and Bass Tablature Editor",
        extensions: &["ptb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x74, 0x61, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
