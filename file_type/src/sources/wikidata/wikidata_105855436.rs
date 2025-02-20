use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855436: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_436,
        source_type: SourceType::Wikidata,
        name: "GeoWorks GEOS font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x53, 0x57, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
