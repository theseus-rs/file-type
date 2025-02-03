use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850619: FileFormat = FileFormat {
    id: 105_850_619,
    source_type: SourceType::Wikidata,
    name: "Cuttlefish eXtended Format",
    extensions: &["cxf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    0x3A, 0x20, 0x75, 0x6E, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x65, 0x64, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
