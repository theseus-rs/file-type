use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206398: FileFormat = FileFormat {
    id: 28_206_398,
    puid: "wikidata/28206398",
    name: "IWC",
    extensions: &["iwc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x57, 0x43, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
