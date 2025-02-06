use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262857: FileFormat = FileFormat {
    id: 111_262_857,
    source_type: SourceType::Wikidata,
    name: "G.711 A-law european telephony format",
    extensions: &["alaw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
