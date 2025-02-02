use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167884: FileFormat = FileFormat {
    id: 29_167_884,
    source_type: SourceType::Wikidata,
    name: "Personal Ancestral File",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
