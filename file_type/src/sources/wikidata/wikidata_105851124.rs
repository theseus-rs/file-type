use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851124: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_124,
        source_type: SourceType::Wikidata,
        name: "Telmac 600 program",
        extensions: &["tmc600"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x54, 0x4D, 0x43, 0x36])],
                },
            }],
        }],
        related_formats: &[],
    },
};
