use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205559: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_559,
        source_type: SourceType::Wikidata,
        name: "Nokia Operator Logo",
        extensions: &["nol"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x4F, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
