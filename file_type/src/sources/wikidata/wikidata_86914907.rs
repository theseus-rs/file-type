use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86914907: FileFormat = FileFormat {
    id: 86_914_907,
    source_type: SourceType::Wikidata,
    name: "IESNA LM-63 Photometric Data File",
    extensions: &["ies"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x45, 0x53, 0x4E, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
