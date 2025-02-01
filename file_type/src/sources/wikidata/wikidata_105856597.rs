use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856597: FileFormat = FileFormat {
    id: 105_856_597,
    puid: "wikidata/105856597",
    name: "Khoros/Cantata Workspace",
    extensions: &["wksp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
