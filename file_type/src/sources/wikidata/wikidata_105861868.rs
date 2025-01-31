use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861868: FileFormat = FileFormat {
    id: 105_861_868,
    puid: "wikidata/105861868",
    name: "Synder SNG-Player module",
    extensions: &["sng"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x59, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
