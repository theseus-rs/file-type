use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865023: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_023,
        source_type: SourceType::Wikidata,
        name: "Altera Pattern Capture Format",
        extensions: &["pcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
