use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863983: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_983,
        source_type: SourceType::Wikidata,
        name: "Mastercam 9 geometry",
        extensions: &["mc9"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x78, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
