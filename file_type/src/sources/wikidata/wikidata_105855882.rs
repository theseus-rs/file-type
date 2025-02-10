use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855882: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_882,
        source_type: SourceType::Wikidata,
        name: "Dan Bricklin's Demo demo (generic)",
        extensions: &["dbd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
