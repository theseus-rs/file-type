use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857028: FileFormat = FileFormat {
    id: 105_857_028,
    source_type: SourceType::Wikidata,
    name: "Mind Games - Chess saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x45, 0x53, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
