use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860577: FileFormat = FileFormat {
    id: 105_860_577,
    source_type: SourceType::Wikidata,
    name: "LEGO Mindstorms EV3 brick executable code",
    extensions: &["rbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x45, 0x47, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
