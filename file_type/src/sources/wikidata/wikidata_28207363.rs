use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207363: FileFormat = FileFormat {
    id: 28_207_363,
    source_type: SourceType::Wikidata,
    name: "TealPaint PDB",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
