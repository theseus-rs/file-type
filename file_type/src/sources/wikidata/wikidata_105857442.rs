use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857442: FileFormat = FileFormat {
    id: 105_857_442,
    puid: "wikidata/105857442",
    name: "3by5 source",
    extensions: &["3x5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x62, 0x79, 0x35, 0x20, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
