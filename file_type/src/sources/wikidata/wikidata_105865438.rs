use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_438,
        source_type: SourceType::Wikidata,
        name: "Prorunner 2.0 Music",
        extensions: &["pru2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x54, 0x21, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
