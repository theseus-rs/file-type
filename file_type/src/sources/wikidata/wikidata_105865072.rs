use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865072: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_072,
        source_type: SourceType::Wikidata,
        name: "PxTone Collage module",
        extensions: &["ptcop"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x54, 0x43, 0x4F, 0x4C, 0x4C, 0x41, 0x47, 0x45, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
