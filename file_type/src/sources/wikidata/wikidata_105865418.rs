use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865418: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_418,
        source_type: SourceType::Wikidata,
        name: "Starbound game data archive",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x42, 0x41, 0x73, 0x73, 0x65, 0x74, 0x36,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
