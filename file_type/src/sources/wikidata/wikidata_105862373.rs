use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862373: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_373,
        source_type: SourceType::Wikidata,
        name: "MegaZeux game",
        extensions: &["mzx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
