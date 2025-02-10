use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850431: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_431,
        source_type: SourceType::Wikidata,
        name: "Royal Heroes game data archive",
        extensions: &["car"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x61, 0x63, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
