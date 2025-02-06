use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207363: FileFormat = FileFormat {
    id: 28_207_363,
    source_type: SourceType::Wikidata,
    name: "TealPaint PDB",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
