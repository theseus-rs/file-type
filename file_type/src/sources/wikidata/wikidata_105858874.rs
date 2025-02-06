use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858874: FileFormat = FileFormat {
    id: 105_858_874,
    source_type: SourceType::Wikidata,
    name: "Lepton bitmap (zlib compressed)",
    extensions: &["lep"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
