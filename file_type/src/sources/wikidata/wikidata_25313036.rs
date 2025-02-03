use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25313036: FileFormat = FileFormat {
    id: 25_313_036,
    source_type: SourceType::Wikidata,
    name: "Extensible Data Notation",
    extensions: &["edn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
