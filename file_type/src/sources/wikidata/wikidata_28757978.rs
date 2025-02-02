use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757978: FileFormat = FileFormat {
    id: 28_757_978,
    source_type: SourceType::Wikidata,
    name: "Precompiled Windows Setup Information File",
    extensions: &["pnf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
