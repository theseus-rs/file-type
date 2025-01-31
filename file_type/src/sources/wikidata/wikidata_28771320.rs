use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771320: FileFormat = FileFormat {
    id: 28_771_320,
    puid: "wikidata/28771320",
    name: "Microsoft Office File List",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
