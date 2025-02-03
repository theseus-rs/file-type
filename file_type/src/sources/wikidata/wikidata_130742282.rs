use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130742282: FileFormat = FileFormat {
    id: 130_742_282,
    source_type: SourceType::Wikidata,
    name: "scdoc file format",
    extensions: &["scd", "scdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
