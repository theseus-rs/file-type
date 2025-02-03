use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757998: FileFormat = FileFormat {
    id: 28_757_998,
    source_type: SourceType::Wikidata,
    name: "Inflate",
    extensions: &["infl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
