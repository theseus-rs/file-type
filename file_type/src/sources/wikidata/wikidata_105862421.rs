use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862421: FileFormat = FileFormat {
    id: 105_862_421,
    source_type: SourceType::Wikidata,
    name: "Symphonie Module",
    extensions: &["symmod"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x79, 0x6D, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
