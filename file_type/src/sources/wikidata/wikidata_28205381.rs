use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205381: FileFormat = FileFormat {
    id: 28_205_381,
    source_type: SourceType::Wikidata,
    name: "Lytro",
    extensions: &["lfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
