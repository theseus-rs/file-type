use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27486884: FileType = FileType {
    file_format: &FileFormat {
        id: 27_486_884,
        source_type: SourceType::Wikidata,
        name: "Shapefile main file",
        extensions: &["shp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x27, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
