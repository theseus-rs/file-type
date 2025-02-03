use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853572: FileFormat = FileFormat {
    id: 105_853_572,
    source_type: SourceType::Wikidata,
    name: "Atlantis Word Processor Lexicon",
    extensions: &["zlx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x4C, 0xE5, 0xBF])],
            },
        }],
    }],
    related_formats: &[],
};
