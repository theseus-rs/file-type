use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60371646: FileFormat = FileFormat {
    id: 60_371_646,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Data File, version 10",
    extensions: &["qcd", "qct", "qtp", "qxp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
