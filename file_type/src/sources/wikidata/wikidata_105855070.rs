use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855070: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_070,
        source_type: SourceType::Wikidata,
        name: "ZX Audio",
        extensions: &["azx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x41, 0x75, 0x64, 0x69, 0x6F, 0x21, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
