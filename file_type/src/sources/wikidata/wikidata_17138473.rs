use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17138473: FileType = FileType {
    file_format: &FileFormat {
        id: 17_138_473,
        source_type: SourceType::Wikidata,
        name: "Sun Raster",
        extensions: &[
            "im1", "im24", "im32", "im8", "ras", "rast", "rs", "scr", "sr", "sun",
        ],
        media_types: &["image/x-sun-raster"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0xA6, 0x6A, 0x95])],
                },
            }],
        }],
        related_formats: &[],
    },
};
