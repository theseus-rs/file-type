use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_735,
        source_type: SourceType::Wikidata,
        name: "SequoiaView directory Tree (XML)",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6E, 0x6F, 0x64, 0x65, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x20, 0x3D,
                        0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
