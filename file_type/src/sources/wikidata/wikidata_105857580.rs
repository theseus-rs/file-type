use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_580,
        source_type: SourceType::Wikidata,
        name: "IRIX Arena data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0xAD, 0xBA, 0xBE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
