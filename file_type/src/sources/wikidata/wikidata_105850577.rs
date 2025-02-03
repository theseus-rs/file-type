use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850577: FileFormat = FileFormat {
    id: 105_850_577,
    source_type: SourceType::Wikidata,
    name: "CorelDRAW for OS/2 drawing (v2.5)",
    extensions: &["cdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x4C, 0xDC, 0x00, 0x25, 0x05])],
            },
        }],
    }],
    related_formats: &[],
};
