use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61963304: FileFormat = FileFormat {
    id: 61_963_304,
    puid: "wikidata/61963304",
    name: "Microsoft Front Page Binary Tree Index",
    extensions: &["btr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
