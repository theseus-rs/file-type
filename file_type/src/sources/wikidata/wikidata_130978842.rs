use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130978842: FileFormat = FileFormat {
    id: 130_978_842,
    source_type: SourceType::Wikidata,
    name: "Slash file format",
    extensions: &["sla"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
