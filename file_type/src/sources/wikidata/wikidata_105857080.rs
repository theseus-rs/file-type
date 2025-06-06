use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_080,
        source_type: SourceType::Wikidata,
        name: "GFA-BASIC Atari v3.07 tokenized source (protected)",
        extensions: &["gfa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x03, 0x47, 0x46, 0x41, 0x2D, 0x42, 0x41, 0x53, 0x49, 0x43, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
