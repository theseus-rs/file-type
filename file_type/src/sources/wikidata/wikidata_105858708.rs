use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858708: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_708,
        source_type: SourceType::Wikidata,
        name: "Lepton UJG bitmap",
        extensions: &["ujg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x4A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
