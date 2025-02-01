use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120966130: FileFormat = FileFormat {
    id: 120_966_130,
    puid: "wikidata/120966130",
    name: "Microsoft Money 97 data",
    extensions: &["mn5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
