use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59962263: FileFormat = FileFormat {
    id: 59_962_263,
    source_type: SourceType::Wikidata,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
