use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859712: FileFormat = FileFormat {
    id: 105_859_712,
    source_type: SourceType::Wikidata,
    name: "VisualBasic Project (ActiveX DLL)",
    extensions: &["vbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x79, 0x70, 0x65, 0x3D, 0x4F, 0x6C, 0x65, 0x44, 0x6C, 0x6C, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
