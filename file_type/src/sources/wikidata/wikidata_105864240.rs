use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864240: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_240,
        source_type: SourceType::Wikidata,
        name: "Palm TealDoc document",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x54, 0x45, 0x58, 0x74, 0x54, 0x6C, 0x44, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
