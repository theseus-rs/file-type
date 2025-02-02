use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130978842: FileFormat = FileFormat {
    id: 130_978_842,
    source_type: SourceType::Wikidata,
    name: "Slash file format",
    extensions: &["sla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
