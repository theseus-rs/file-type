use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850370: FileFormat = FileFormat {
    id: 105_850_370,
    source_type: SourceType::Wikidata,
    name: "Cult3D object",
    extensions: &["co"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x33, 0x44, 0x46, 0x30, 0x39, 0x34, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
