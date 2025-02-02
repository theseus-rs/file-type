use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862915: FileFormat = FileFormat {
    id: 105_862_915,
    source_type: SourceType::Wikidata,
    name: "GNU Octave Matrix data (text)",
    extensions: &["mat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
                    0x4F, 0x63, 0x74, 0x61, 0x76, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
