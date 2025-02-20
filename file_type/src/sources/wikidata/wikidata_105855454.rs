use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855454: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_454,
        source_type: SourceType::Wikidata,
        name: "FleetStreet Installation archive",
        extensions: &["fil"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7E, 0x04, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
