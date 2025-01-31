use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51370102: FileFormat = FileFormat {
    id: 51_370_102,
    puid: "wikidata/51370102",
    name: "Microsoft Print File",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
