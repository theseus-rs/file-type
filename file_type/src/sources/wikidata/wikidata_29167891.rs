use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167891: FileFormat = FileFormat {
    id: 29_167_891,
    source_type: SourceType::Wikidata,
    name: "Personal Ancestral File, version 4",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
