use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979389: FileFormat = FileFormat {
    id: 27_979_389,
    puid: "wikidata/27979389",
    name: "NEOchrome Animation",
    extensions: &["ani"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBA, 0xBE, 0xEB, 0xEA])],
            },
        }],
    }],
    related_formats: &[],
};
