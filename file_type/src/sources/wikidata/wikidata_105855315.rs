use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855315: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_315,
        source_type: SourceType::Wikidata,
        name: "Westwood Font Format (v4)",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x00, 0x0E, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
