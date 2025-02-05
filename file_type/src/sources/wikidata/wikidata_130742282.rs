use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130742282: FileFormat = FileFormat {
    id: 130_742_282,
    source_type: SourceType::Wikidata,
    name: "scdoc file format",
    extensions: &["scd", "scdoc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
