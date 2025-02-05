use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857991: FileFormat = FileFormat {
    id: 105_857_991,
    source_type: SourceType::Wikidata,
    name: "Sonic Global Image",
    extensions: &["gi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDA, 0xDA, 0xFE, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
