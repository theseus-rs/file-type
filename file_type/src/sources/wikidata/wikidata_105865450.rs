use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865450: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_450,
        source_type: SourceType::Wikidata,
        name: "Protocol Data Unit message data",
        extensions: &["pdu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x8C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
