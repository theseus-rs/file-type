use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850160: FileFormat = FileFormat {
    id: 105_850_160,
    source_type: SourceType::Wikidata,
    name: "Abomination: The Nemesis Project game data archive",
    extensions: &["clt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x57, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
