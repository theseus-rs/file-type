use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864343: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_343,
        source_type: SourceType::Wikidata,
        name: "PGP ASCII-Armor",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50,
                        0x47, 0x50, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
