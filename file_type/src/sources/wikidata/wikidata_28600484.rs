use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600484: FileFormat = FileFormat {
    id: 28_600_484,
    puid: "wikidata/28600484",
    name: "DVDisaster Error Correction File",
    extensions: &["ecc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x64, 0x76, 0x64, 0x69, 0x73, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
