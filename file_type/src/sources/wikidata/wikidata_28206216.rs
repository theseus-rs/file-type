use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206216: FileFormat = FileFormat {
    id: 28_206_216,
    puid: "wikidata/28206216",
    name: "GrayPaint",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
