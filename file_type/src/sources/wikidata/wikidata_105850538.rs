use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850538: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_538,
        source_type: SourceType::Wikidata,
        name: "Hidden and Dangerous game data archive",
        extensions: &["cnt", "dta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x50, 0x43, 0x4E, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
