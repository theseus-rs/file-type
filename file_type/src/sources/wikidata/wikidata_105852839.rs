use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852839: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_839,
        source_type: SourceType::Wikidata,
        name: "You Don't Know Jack game data archive",
        extensions: &["srf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x72, 0x66, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
