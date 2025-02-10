use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858764: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_764,
        source_type: SourceType::Wikidata,
        name: "GFA-BASIC Atari v1.00-2.02 tokenized source",
        extensions: &["bas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x47, 0x66, 0x41, 0x42, 0x41, 0x53, 0x49, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
