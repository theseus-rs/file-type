use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66686595: FileFormat = FileFormat {
    id: 66_686_595,
    puid: "wikidata/66686595",
    name: "Microsoft Works Template file format",
    extensions: &["wpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
