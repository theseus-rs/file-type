use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857733: FileFormat = FileFormat {
    id: 105_857_733,
    source_type: SourceType::Wikidata,
    name: "MiSTer settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x69, 0x53, 0x54, 0x65, 0x72, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
