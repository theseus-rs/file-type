use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865422: FileFormat = FileFormat {
    id: 105_865_422,
    source_type: SourceType::Wikidata,
    name: "Protein Databank (with HTML header)",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
