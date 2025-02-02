use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7251429: FileFormat = FileFormat {
    id: 7_251_429,
    source_type: SourceType::Wikidata,
    name: "Protein Data Bank",
    extensions: &["pdb"],
    media_types: &["chemical/x-pdb"],
    internal_signatures: &[],
    related_formats: &[],
};
