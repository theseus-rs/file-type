use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130619766: FileFormat = FileFormat {
    id: 130_619_766,
    source_type: SourceType::Wikidata,
    name: "RelaxNG compact syntax file format",
    extensions: &["rnc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
