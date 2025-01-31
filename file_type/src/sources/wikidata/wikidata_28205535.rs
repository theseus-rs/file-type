use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205535: FileFormat = FileFormat {
    id: 28_205_535,
    puid: "wikidata/28205535",
    name: "iThmb",
    extensions: &["ithmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
