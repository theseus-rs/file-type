use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850929: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_929,
        source_type: SourceType::Wikidata,
        name: "Trivial Data Base",
        extensions: &["tdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x44, 0x42, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
