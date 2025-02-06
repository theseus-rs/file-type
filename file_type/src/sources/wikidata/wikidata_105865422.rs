use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865422: FileFormat = FileFormat {
    id: 105_865_422,
    source_type: SourceType::Wikidata,
    name: "Protein Databank (with HTML header)",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
