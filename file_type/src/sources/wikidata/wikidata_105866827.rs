use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866827: FileFormat = FileFormat {
    id: 105_866_827,
    source_type: SourceType::Wikidata,
    name: "Altium Designer project",
    extensions: &["prjpcb"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
