use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854544: FileFormat = FileFormat {
    id: 105_854_544,
    source_type: SourceType::Wikidata,
    name: "Adobe Digital Editions Adobe Content Server Message",
    extensions: &["acsm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
