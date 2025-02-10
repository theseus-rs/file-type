use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857865: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_865,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine exported player Character (v2.0)",
        extensions: &["chr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x52, 0x20, 0x56, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
