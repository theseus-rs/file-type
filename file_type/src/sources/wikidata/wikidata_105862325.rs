use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862325: FileFormat = FileFormat {
    id: 105_862_325,
    source_type: SourceType::Wikidata,
    name: "Aegis Videoscape 3D Motion",
    extensions: &["mot"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x4D, 0x31, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
