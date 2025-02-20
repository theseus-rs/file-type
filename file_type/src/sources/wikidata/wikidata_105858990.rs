use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858990: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_990,
        source_type: SourceType::Wikidata,
        name: "BigTIFF bitmap",
        extensions: &["tif", "tiff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
