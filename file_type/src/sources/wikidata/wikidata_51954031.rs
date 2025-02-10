use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_51954031: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_031,
        source_type: SourceType::Wikidata,
        name: "MapBrowser/MapWriter Vector Map Data",
        extensions: &["cbd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x77, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
