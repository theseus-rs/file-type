use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009476: FileFormat = FileFormat {
    id: 28_009_476,
    source_type: SourceType::Wikidata,
    name: "SCF",
    extensions: &["scf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
