use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967402: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_402,
        source_type: SourceType::Wikidata,
        name: "Beni Tracker module",
        extensions: &["pis"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
