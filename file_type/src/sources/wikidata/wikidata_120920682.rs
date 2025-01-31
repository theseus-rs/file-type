use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920682: FileFormat = FileFormat {
    id: 120_920_682,
    puid: "wikidata/120920682",
    name: "Microsoft Money 2004 backup data",
    extensions: &["m12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
