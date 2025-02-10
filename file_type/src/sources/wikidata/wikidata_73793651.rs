use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_73793651: FileType = FileType {
    file_format: &FileFormat {
        id: 73_793_651,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro add-in functions library",
        extensions: &["qll"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x44, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
