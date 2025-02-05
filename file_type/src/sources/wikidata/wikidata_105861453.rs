use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861453: FileFormat = FileFormat {
    id: 105_861_453,
    source_type: SourceType::Wikidata,
    name: "Laser Compact compressed screen",
    extensions: &["lc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x43, 0x4D, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
