use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857053: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_053,
        source_type: SourceType::Wikidata,
        name: "Chasys Draw IES Gradient",
        extensions: &["gra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x52, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
