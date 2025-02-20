use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967407: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_407,
        source_type: SourceType::Wikidata,
        name: "Surprise! Adlib Tracker version 2.0",
        extensions: &["sa2"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
