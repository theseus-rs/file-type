use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856955: FileFormat = FileFormat {
    id: 105_856_955,
    puid: "wikidata/105856955",
    name: "ExpressGraph Graph",
    extensions: &["grf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x52, 0x41, 0x50, 0x48, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
