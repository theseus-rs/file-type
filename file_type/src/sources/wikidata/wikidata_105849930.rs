use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_930,
        source_type: SourceType::Wikidata,
        name: "Chasys Draw IES drawing",
        extensions: &["cd5"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5F, 0x43, 0x44, 0x35])],
                },
            }],
        }],
        related_formats: &[],
    },
};
