use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863643: FileFormat = FileFormat {
    id: 105_863_643,
    source_type: SourceType::Wikidata,
    name: "MaxTrax module",
    extensions: &["mxtx"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x58, 0x54, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
