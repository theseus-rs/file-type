use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857225: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_225,
        source_type: SourceType::Wikidata,
        name: "Peter Norton Computing Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4E, 0x43, 0x49, 0x42, 0x48, 0x44, 0x4D, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
