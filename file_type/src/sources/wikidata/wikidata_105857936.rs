use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_936,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine save Game (v2.2)",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x41, 0x4D, 0x45, 0x56, 0x32, 0x2E, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
