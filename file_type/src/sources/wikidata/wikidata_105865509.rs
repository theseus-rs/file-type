use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_509,
        source_type: SourceType::Wikidata,
        name: "Pixel Bender bytecode",
        extensions: &["pbj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA5, 0x01, 0x00, 0x00, 0x00, 0xA4])],
                },
            }],
        }],
        related_formats: &[],
    },
};
