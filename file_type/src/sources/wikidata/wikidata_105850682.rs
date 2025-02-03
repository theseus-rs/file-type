use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850682: FileFormat = FileFormat {
    id: 105_850_682,
    source_type: SourceType::Wikidata,
    name: "Prisoner Of Ice game data archive",
    extensions: &["kro"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x75, 0x72, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
