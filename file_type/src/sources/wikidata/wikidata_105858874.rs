use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858874: FileFormat = FileFormat {
    id: 105_858_874,
    puid: "wikidata/105858874",
    name: "Lepton bitmap (zlib compressed)",
    extensions: &["lep"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCE, 0xB6])],
            },
        }],
    }],
    related_formats: &[],
};
