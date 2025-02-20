use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864891: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_891,
        source_type: SourceType::Wikidata,
        name: "Personal Media Suite encoded file",
        extensions: &["pms"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x09, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
