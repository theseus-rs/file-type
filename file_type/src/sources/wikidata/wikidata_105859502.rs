use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859502: FileFormat = FileFormat {
    id: 105_859_502,
    source_type: SourceType::Wikidata,
    name: "Vampire Engine MageSlayer game data archive",
    extensions: &["vpk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x67, 0x65, 0x44, 0x65, 0x6D, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
