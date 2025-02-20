use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_384,
        source_type: SourceType::Wikidata,
        name: "Avira AntiVir quarantined",
        extensions: &["qua"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6E, 0x74, 0x69, 0x56, 0x69, 0x72, 0x20, 0x51, 0x75, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
