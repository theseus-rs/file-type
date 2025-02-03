use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60371646: FileFormat = FileFormat {
    id: 60_371_646,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Data File, version 10",
    extensions: &["qcd", "qct", "qtp", "qxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
