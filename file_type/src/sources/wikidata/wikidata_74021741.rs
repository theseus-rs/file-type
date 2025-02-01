use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74021741: FileFormat = FileFormat {
    id: 74_021_741,
    puid: "wikidata/74021741",
    name: "Microsoft Resource Library",
    extensions: &["rll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
