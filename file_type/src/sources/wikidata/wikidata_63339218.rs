use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63339218: FileFormat = FileFormat {
    id: 63_339_218,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 4.5",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
