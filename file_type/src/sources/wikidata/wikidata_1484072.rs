use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1484072: FileFormat = FileFormat {
    id: 1_484_072,
    puid: "wikidata/1484072",
    name: "ZIX archive",
    extensions: &["zix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x49, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
