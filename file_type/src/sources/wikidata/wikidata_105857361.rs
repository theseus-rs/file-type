use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857361: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_361,
        source_type: SourceType::Wikidata,
        name: "JNI Library",
        extensions: &["jnilib"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
