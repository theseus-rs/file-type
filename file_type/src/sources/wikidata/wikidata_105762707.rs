use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762707: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_707,
        source_type: SourceType::Wikidata,
        name: "LaunchBox games data",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4C, 0x61, 0x75, 0x6E, 0x63, 0x68, 0x42, 0x6F, 0x78, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
