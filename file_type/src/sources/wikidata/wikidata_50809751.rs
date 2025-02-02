use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50809751: FileFormat = FileFormat {
    id: 50_809_751,
    source_type: SourceType::Wikidata,
    name: "Portable Database, version 1",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
