use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049454: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_454,
        source_type: SourceType::Wikidata,
        name: "DEGAS Elite Compressed, medium resolution",
        extensions: &["PC2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
