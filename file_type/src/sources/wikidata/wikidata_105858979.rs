use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858979: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_979,
        source_type: SourceType::Wikidata,
        name: "Tagged Image File Format Bitmap (big endian)",
        extensions: &["tif", "tiff"],
        media_types: &["image/tiff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
