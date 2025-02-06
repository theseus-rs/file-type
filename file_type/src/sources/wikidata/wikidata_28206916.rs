use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206916: FileFormat = FileFormat {
    id: 28_206_916,
    source_type: SourceType::Wikidata,
    name: "Portfolio Graphics",
    extensions: &["pgf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
