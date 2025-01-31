use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3406936: FileFormat = FileFormat {
    id: 3_406_936,
    puid: "wikidata/3406936",
    name: "Program database",
    extensions: &["pdb", "pdb"],
    media_types: &["application/x-ms-pdb", "application/x-ms-pdb"],
    internal_signatures: &[],
    related_formats: &[],
};
