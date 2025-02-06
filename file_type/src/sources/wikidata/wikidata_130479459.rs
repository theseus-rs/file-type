use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130479459: FileFormat = FileFormat {
    id: 130_479_459,
    source_type: SourceType::Wikidata,
    name: "Pony source code file",
    extensions: &["pony"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
