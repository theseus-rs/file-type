use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856802: FileFormat = FileFormat {
    id: 105_856_802,
    source_type: SourceType::Wikidata,
    name: "GeneRally track",
    extensions: &["trk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x47, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
