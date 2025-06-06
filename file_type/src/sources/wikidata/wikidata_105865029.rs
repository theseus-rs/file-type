use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865029: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_029,
        source_type: SourceType::Wikidata,
        name: "Storm Region game data archive",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x72, 0x1A, 0x1C, 0x0D, 0x0A, 0x87, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
