use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_518,
        source_type: SourceType::Wikidata,
        name: "A2R disk image (v2)",
        extensions: &["a2r"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x32, 0x52, 0x32, 0xFF, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
