use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853190: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_190,
        source_type: SourceType::Wikidata,
        name: "Turbo Modula-2 Symbol data",
        extensions: &["sym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x6D, 0x72, 0xE8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
