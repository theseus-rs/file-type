use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865256: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_256,
        source_type: SourceType::Wikidata,
        name: "Pebble Draw Command image",
        extensions: &["pdc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x44, 0x43, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
