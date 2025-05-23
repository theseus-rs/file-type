use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858517: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_517,
        source_type: SourceType::Wikidata,
        name: "Naive Image format NIE bitmap",
        extensions: &["image/nie", "nie"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0xC3, 0xAF, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
