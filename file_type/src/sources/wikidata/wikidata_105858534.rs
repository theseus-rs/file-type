use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_534,
        source_type: SourceType::Wikidata,
        name: "FGF bitmap",
        extensions: &["fgf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x47, 0x46, 0x39, 0x35, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
