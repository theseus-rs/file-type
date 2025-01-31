use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207241: FileFormat = FileFormat {
    id: 28_207_241,
    puid: "wikidata/28207241",
    name: "SBIG CCDOPS image",
    extensions: &["sbig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
