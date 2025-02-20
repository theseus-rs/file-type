use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7598235: FileType = FileType {
    file_format: &FileFormat {
        id: 7_598_235,
        source_type: SourceType::Wikidata,
        name: "Standard Flowgram Format",
        extensions: &["sff"],
        media_types: &["chemical/seq-na-sff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2E, 0x73, 0x66, 0x66, 0x30, 0x30, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
