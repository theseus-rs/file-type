use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967206: FileFormat = FileFormat {
    id: 27_967_206,
    source_type: SourceType::Wikidata,
    name: "Onyx Music File",
    extensions: &["omf"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x6E, 0x79, 0x78, 0x20, 0x4D, 0x75, 0x73, 0x69, 0x63, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
