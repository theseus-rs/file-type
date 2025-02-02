use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3579577: FileFormat = FileFormat {
    id: 3_579_577,
    source_type: SourceType::Wikidata,
    name: "JTAG Chain Information",
    extensions: &["jci"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4A, 0x54, 0x41, 0x47, 0x2D, 0x43, 0x68, 0x61, 0x69, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
