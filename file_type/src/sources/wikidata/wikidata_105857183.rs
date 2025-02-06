use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857183: FileFormat = FileFormat {
    id: 105_857_183,
    source_type: SourceType::Wikidata,
    name: "Heroes of Might and Magic IV game data archive",
    extensions: &["h4r"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x52, 0x82, 0x05])],
            },
        }],
    }],
    related_formats: &[],
};
