use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857469: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_469,
        source_type: SourceType::Wikidata,
        name: "3Digi Parameters",
        extensions: &["3dp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x61, 0x72, 0x53, 0x65, 0x74, 0x30, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
