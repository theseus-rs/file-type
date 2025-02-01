use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110458180: FileFormat = FileFormat {
    id: 110_458_180,
    puid: "wikidata/110458180",
    name: "Beam Software SIFF File",
    extensions: &["son", "vb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
