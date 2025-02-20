use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850362: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_362,
        source_type: SourceType::Wikidata,
        name: "CHX font format",
        extensions: &["chx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x58, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
