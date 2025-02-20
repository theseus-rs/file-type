use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850421: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_421,
        source_type: SourceType::Wikidata,
        name: "Cal3D morphanimation File",
        extensions: &["cpf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
