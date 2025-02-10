use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851899: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_899,
        source_type: SourceType::Wikidata,
        name: "A'dam Music Composer Script",
        extensions: &["scr"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x65, 0x74, 0x20, 0x74, 0x65, 0x6D, 0x70, 0x6F, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
