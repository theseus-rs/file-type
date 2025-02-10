use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863608: FileFormat = FileFormat {
    id: 105_863_608,
    source_type: SourceType::Wikidata,
    name: "MoonBlaster for MoonSound song",
    extensions: &["mfm", "mwk", "mwm", "mwv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x42, 0x4D, 0x53, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
