use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850690: FileFormat = FileFormat {
    id: 105_850_690,
    source_type: SourceType::Wikidata,
    name: "Kaitai Struct language (with rem)",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
