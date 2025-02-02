use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7118753: FileFormat = FileFormat {
    id: 7_118_753,
    source_type: SourceType::Wikidata,
    name: "PDB",
    extensions: &["pdb"],
    media_types: &["application/vnd.palm"],
    internal_signatures: &[],
    related_formats: &[],
};
