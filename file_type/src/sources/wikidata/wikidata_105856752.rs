use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856752: FileFormat = FileFormat {
    id: 105_856_752,
    source_type: SourceType::Wikidata,
    name: "JB BAHN scenery (Zoom2)",
    extensions: &["uz2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x42, 0x20, 0x42, 0x41, 0x48, 0x4E, 0x20, 0x53, 0x63, 0x65, 0x6E, 0x65,
                    0x72, 0x79, 0x20, 0x67, 0x66, 0x78, 0x20, 0x5A, 0x6F, 0x6F, 0x6D, 0x32, 0x0D,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
