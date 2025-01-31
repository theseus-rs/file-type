use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859509: FileFormat = FileFormat {
    id: 105_859_509,
    puid: "wikidata/105859509",
    name: "PVA Video (VideoStream)",
    extensions: &["pva"],
    media_types: &["video/x-pva"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x56, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
