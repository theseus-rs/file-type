use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859016: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_016,
        source_type: SourceType::Wikidata,
        name: "PrintPartner Banner",
        extensions: &["ban"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7E, 0x2A, 0x21, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
