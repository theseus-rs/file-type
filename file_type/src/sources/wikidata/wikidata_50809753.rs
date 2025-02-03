use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50809753: FileFormat = FileFormat {
    id: 50_809_753,
    source_type: SourceType::Wikidata,
    name: "Portable Database, version 2",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
