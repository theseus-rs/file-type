use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263007: FileFormat = FileFormat {
    id: 111_263_007,
    source_type: SourceType::Wikidata,
    name: "Velvet Studio sample",
    extensions: &["ase"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
