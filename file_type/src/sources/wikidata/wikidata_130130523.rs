use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130130523: FileFormat = FileFormat {
    id: 130_130_523,
    source_type: SourceType::Wikidata,
    name: "Kuin source code file",
    extensions: &["kn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
