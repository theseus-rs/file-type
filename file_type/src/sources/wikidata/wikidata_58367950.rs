use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58367950: FileFormat = FileFormat {
    id: 58_367_950,
    puid: "wikidata/58367950",
    name: "Microsoft Project file format version 2007",
    extensions: &["mpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
