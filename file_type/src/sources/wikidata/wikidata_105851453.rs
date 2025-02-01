use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851453: FileFormat = FileFormat {
    id: 105_851_453,
    puid: "wikidata/105851453",
    name: "DDP Image checksums",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x52, 0x43, 0x33, 0x32, 0x20, 0x43, 0x68, 0x65, 0x63, 0x6B, 0x73,
                    0x75, 0x6D, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
