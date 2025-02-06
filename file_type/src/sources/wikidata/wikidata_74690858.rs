use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74690858: FileFormat = FileFormat {
    id: 74_690_858,
    source_type: SourceType::Wikidata,
    name: "TomeRaider 3 eBook",
    extensions: &["tr3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x33, 0x44, 0x54, 0x52, 0x33, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
