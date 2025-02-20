use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850708: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_708,
        source_type: SourceType::Wikidata,
        name: "ProHance Mouse Keys Definition table",
        extensions: &["kdh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x9C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
