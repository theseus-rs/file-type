use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207256: FileFormat = FileFormat {
    id: 28_207_256,
    puid: "wikidata/28207256",
    name: "ScreenShot Hack PDB",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
