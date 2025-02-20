use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859270: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_270,
        source_type: SourceType::Wikidata,
        name: "Tagged Image File Format Bitmap (little endian)",
        extensions: &["tif", "tiff"],
        media_types: &["image/tiff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
