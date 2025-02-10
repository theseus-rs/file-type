use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856857: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_857,
        source_type: SourceType::Wikidata,
        name: "GFA-BASIC Atari v2 tokenized source (protected)",
        extensions: &["gfa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x02, 0x47, 0x66, 0x41, 0x42, 0x41, 0x53, 0x49, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
