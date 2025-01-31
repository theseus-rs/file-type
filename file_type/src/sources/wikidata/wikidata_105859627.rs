use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859627: FileFormat = FileFormat {
    id: 105_859_627,
    puid: "wikidata/105859627",
    name: "Visual Studio Setup and Deployment Project",
    extensions: &["vdproj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x22, 0x44, 0x65, 0x70, 0x6C, 0x6F, 0x79, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                    0x74, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
