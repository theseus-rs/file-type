use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852718: FileFormat = FileFormat {
    id: 105_852_718,
    puid: "wikidata/105852718",
    name: "NGS orbital format SP3 (positions only)",
    extensions: &["sp3"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x61, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
