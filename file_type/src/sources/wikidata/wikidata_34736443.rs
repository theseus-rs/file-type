use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34736443: FileType = FileType {
    file_format: &FileFormat {
        id: 34_736_443,
        source_type: SourceType::Wikidata,
        name: "Simple Vector Format, version 2",
        extensions: &["svf"],
        media_types: &["image/vnd.svf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x56, 0x46, 0x20, 0x76, 0x32, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
