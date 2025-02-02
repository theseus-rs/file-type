use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207256: FileFormat = FileFormat {
    id: 28_207_256,
    source_type: SourceType::Wikidata,
    name: "ScreenShot Hack PDB",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
