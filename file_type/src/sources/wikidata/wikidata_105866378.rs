use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866378: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_378,
        source_type: SourceType::Wikidata,
        name: "Post-It Software Note Template",
        extensions: &["psntemplate"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6E, 0x6F, 0x74, 0x65, 0x3E, 0x3C, 0x63, 0x6C, 0x73, 0x69, 0x64,
                        0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
