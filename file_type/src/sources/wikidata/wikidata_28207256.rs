use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207256: FileFormat = FileFormat {
    id: 28_207_256,
    source_type: SourceType::Wikidata,
    name: "ScreenShot Hack PDB",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
