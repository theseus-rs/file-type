use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60372734: FileFormat = FileFormat {
    id: 60_372_734,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Data File, version 6",
    extensions: &["qcd", "qct", "qtp", "qxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
