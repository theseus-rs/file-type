use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860994: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_994,
        source_type: SourceType::Wikidata,
        name: "Footprint/IBM Works Data Filer DataBase",
        extensions: &["ldb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x8B, 0x5D, 0x09])],
                },
            }],
        }],
        related_formats: &[],
    },
};
