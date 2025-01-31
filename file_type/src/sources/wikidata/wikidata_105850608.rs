use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850608: FileFormat = FileFormat {
    id: 105_850_608,
    puid: "wikidata/105850608",
    name: "Chessmaster saved game (generic)",
    extensions: &["cmg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x65, 0x73, 0x73, 0x6D, 0x61, 0x73, 0x74, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
