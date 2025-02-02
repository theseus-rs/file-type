use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7375542: FileFormat = FileFormat {
    id: 7_375_542,
    source_type: SourceType::Wikidata,
    name: "Restricted Permission Message",
    extensions: &["rpmsg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0xE8, 0x04, 0x60, 0xC4, 0x11, 0xE3, 0x86,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
