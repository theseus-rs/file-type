use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850170: FileFormat = FileFormat {
    id: 105_850_170,
    source_type: SourceType::Wikidata,
    name: "Cheat Engine Cheat Table",
    extensions: &["ct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x45, 0x41, 0x54, 0x45, 0x4E, 0x47, 0x49, 0x4E, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
