use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48625328: FileFormat = FileFormat {
    id: 48_625_328,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript, v2",
    extensions: &["eps", "epsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
