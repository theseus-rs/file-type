use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7118753: FileFormat = FileFormat {
    id: 7_118_753,
    puid: "wikidata/7118753",
    name: "PDB",
    extensions: &["pdb"],
    media_types: &["application/vnd.palm"],
    internal_signatures: &[],
    related_formats: &[],
};
