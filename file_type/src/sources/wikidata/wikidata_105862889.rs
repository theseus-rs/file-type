use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862889: FileFormat = FileFormat {
    id: 105_862_889,
    puid: "wikidata/105862889",
    name: "MacStitch/WinStitch Motif",
    extensions: &["motif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x22, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x4D, 0x6F, 0x74, 0x69, 0x66,
                    0x20, 0x56, 0x31, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
