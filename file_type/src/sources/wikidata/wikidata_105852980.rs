use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852980: FileFormat = FileFormat {
    id: 105_852_980,
    puid: "wikidata/105852980",
    name: "Mocha Snapshot",
    extensions: &["snp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0xED])],
            },
        }],
    }],
    related_formats: &[],
};
