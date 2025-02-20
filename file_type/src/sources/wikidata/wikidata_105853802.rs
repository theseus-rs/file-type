use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853802: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_802,
        source_type: SourceType::Wikidata,
        name: "AKT compressed archive",
        extensions: &["akt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4B, 0x54, 0x09, 0x09])],
                },
            }],
        }],
        related_formats: &[],
    },
};
