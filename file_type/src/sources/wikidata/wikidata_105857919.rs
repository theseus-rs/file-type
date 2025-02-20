use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857919: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_919,
        source_type: SourceType::Wikidata,
        name: "Img3 enctrypted signed container",
        extensions: &["img3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x67, 0x6D, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
