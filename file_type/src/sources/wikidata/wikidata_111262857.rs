use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262857: FileFormat = FileFormat {
    id: 111_262_857,
    source_type: SourceType::Wikidata,
    name: "G.711 A-law european telephony format",
    extensions: &["alaw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
