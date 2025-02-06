use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863726: FileFormat = FileFormat {
    id: 105_863_726,
    source_type: SourceType::Wikidata,
    name: "Monotone Tracker chiptune",
    extensions: &["mon"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x08, 0x4D, 0x4F, 0x4E, 0x4F, 0x54, 0x4F, 0x4E, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
