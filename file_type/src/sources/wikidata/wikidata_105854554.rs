use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854554: FileFormat = FileFormat {
    id: 105_854_554,
    source_type: SourceType::Wikidata,
    name: "PS/2 MicroChannel Adapter Description File",
    extensions: &["adf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x64, 0x61, 0x70, 0x74, 0x65, 0x72])],
            },
        }],
    }],
    related_formats: &[],
};
