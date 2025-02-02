use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850257: FileFormat = FileFormat {
    id: 105_850_257,
    source_type: SourceType::Wikidata,
    name: "EGrid32 Compilable Grid Format (ready to be modified)",
    extensions: &["cgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x47, 0x33, 0x32, 0x50, 0x52, 0x4F, 0x5F, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
