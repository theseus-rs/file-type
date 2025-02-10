use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850882: FileFormat = FileFormat {
    id: 105_850_882,
    source_type: SourceType::Wikidata,
    name: "Kazaa Playlist",
    extensions: &["kpl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
