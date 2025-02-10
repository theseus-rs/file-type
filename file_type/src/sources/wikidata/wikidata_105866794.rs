use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_794,
        source_type: SourceType::Wikidata,
        name: "Micrografx Designer Palette",
        extensions: &["pl4"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x00, 0x01, 0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
