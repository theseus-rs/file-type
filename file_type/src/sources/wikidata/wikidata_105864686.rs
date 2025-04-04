use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864686: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_686,
        source_type: SourceType::Wikidata,
        name: "Phoenix RC simulator update",
        extensions: &["phx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x48, 0x4F, 0x45, 0x4E, 0x49, 0x58, 0x5F, 0x55, 0x50, 0x44, 0x41,
                        0x54, 0x45, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
