use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130288276: FileFormat = FileFormat {
    id: 130_288_276,
    puid: "wikidata/130288276",
    name: "MYTHSAV",
    extensions: &["mythsav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
