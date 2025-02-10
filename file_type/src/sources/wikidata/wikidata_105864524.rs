use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864524: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_524,
        source_type: SourceType::Wikidata,
        name: "SpongeBob SquarePants: Lights,Camera,Pants! game data archive",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6B, 0x63, 0x61, 0x70, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
