use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863733: FileFormat = FileFormat {
    id: 105_863_733,
    source_type: SourceType::Wikidata,
    name: "Octalyser 6-channel STe/Falcon Module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x36, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
