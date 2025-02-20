use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850415: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_415,
        source_type: SourceType::Wikidata,
        name: "Clip Gallery Download Package",
        extensions: &["cil"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x00, 0x50, 0x00, 0x4C, 0x00, 0x49, 0x00, 0x54, 0x00, 0x43, 0x00,
                        0x49, 0x00, 0x4C, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
