use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856435: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_435,
        source_type: SourceType::Wikidata,
        name: "GFA Raytrace Animation (hi-res)",
        extensions: &["wah"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x61, 0x68, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
