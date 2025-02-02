use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_57978134: FileFormat = FileFormat {
    id: 57_978_134,
    source_type: SourceType::Wikidata,
    name: "ASP Control Directive File",
    extensions: &["ascx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x25, 0x40, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
